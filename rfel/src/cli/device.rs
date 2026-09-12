//! Host-side device discovery, interactive selection, and project preferences.

use std::env;
use std::fs;
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};

use crossterm::{
    cursor::{MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use dialoguer::Select;
use nusb::{DeviceInfo, MaybeFuture};
use toml_edit::{DocumentMut, value};

use super::CliError;
use crate::fel::Fel;

const CONFIG_NAME: &str = "Rfel.toml";

/// Restore the original terminal on selection, cancellation, or an IO error.
struct SelectionScreen;

impl SelectionScreen {
    fn enter() -> Result<Self, CliError> {
        execute!(std::io::stderr(), EnterAlternateScreen)
            .map_err(|err| failure(format!("cannot enter device selection screen: {err}")))?;
        let screen = Self;
        execute!(std::io::stderr(), Clear(ClearType::All), MoveTo(0, 0))
            .map_err(|err| failure(format!("cannot clear device selection screen: {err}")))?;
        Ok(screen)
    }
}

impl Drop for SelectionScreen {
    fn drop(&mut self) {
        let _ = execute!(std::io::stderr(), Show, LeaveAlternateScreen);
    }
}

fn failure(message: impl Into<String>) -> CliError {
    CliError::Selection(message.into())
}

/// Use Windows location paths, or dfu-util's BUS-PORT.PORT convention.
/// nusb extracts a Windows bus ID through USBROOT from the native location path.
fn topology_selector(bus: &str, ports: &[u8]) -> String {
    if bus.contains("#USBROOT(") {
        let mut location = bus.to_string();
        for port in ports {
            use std::fmt::Write;
            write!(location, "#USB({port})").unwrap();
        }
        return location;
    }
    // Linux nusb bus IDs have zero padding; sysfs and dfu-util paths do not.
    let bus = bus
        .parse::<u8>()
        .map(|bus| bus.to_string())
        .unwrap_or_else(|_| bus.to_string());
    let ports = ports
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(".");
    format!("{bus}-{ports}")
}

pub(super) fn selector(info: &DeviceInfo) -> String {
    topology_selector(info.bus_id(), info.port_chain())
}

fn version(info: &DeviceInfo) -> Result<crate::fel::Version, CliError> {
    let device = info.open().wait().map_err(CliError::OpenDevice)?;
    let mut interface = device
        .claim_interface(0)
        .wait()
        .map_err(CliError::ClaimInterface)?;
    let fel = Fel::open_interface(&mut interface).map_err(CliError::Fel)?;
    // This request needs no SRAM helper and does not read SID.
    fel.get_version().map_err(CliError::Fel)
}

fn description(info: &DeviceInfo) -> String {
    format!(
        "{} (bus {:?}, address {}, ports {:?})",
        selector(info),
        info.bus_id(),
        info.device_address(),
        info.port_chain()
    )
}

pub(super) fn show_versions(devices: &[DeviceInfo]) -> Result<(), CliError> {
    let mut failed = false;
    for info in devices {
        println!("{}", description(info));
        match version(info) {
            Ok(version) => println!("  {version:x?}"),
            Err(err) => {
                failed = true;
                println!("  error: {err}");
            }
        }
    }
    if failed {
        Err(failure(
            "could not read the version of one or more FEL devices",
        ))
    } else {
        Ok(())
    }
}

fn config_path(cwd: &Path) -> PathBuf {
    for dir in cwd.ancestors() {
        let config = dir.join(CONFIG_NAME);
        if config.exists() || dir.join(".git").exists() {
            return config;
        }
    }
    cwd.join(CONFIG_NAME)
}

fn read_config(path: &Path) -> Result<DocumentMut, CliError> {
    match fs::read_to_string(path) {
        Ok(text) => text
            .parse()
            .map_err(|err| failure(format!("invalid {}: {err}", path.display()))),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(DocumentMut::new()),
        Err(err) => Err(failure(format!("cannot read {}: {err}", path.display()))),
    }
}

fn configured_selector(config: &DocumentMut) -> Result<Option<&str>, CliError> {
    config
        .get("device")
        .map(|item| {
            item.as_str()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| failure("Rfel.toml: device must be a non-empty selector string"))
        })
        .transpose()
}

fn match_selector(selectors: &[String], requested: &str) -> Result<Option<usize>, CliError> {
    let mut matches = selectors
        .iter()
        .enumerate()
        .filter(|(_, item)| item.as_str() == requested);
    let first = matches.next().map(|(index, _)| index);
    if matches.next().is_some() {
        return Err(failure(format!(
            "device selector {requested:?} is ambiguous"
        )));
    }
    Ok(first)
}

pub(super) struct Selection {
    pub index: usize,
    save_path: Option<PathBuf>,
}

fn explicit_selection(
    selectors: &[String],
    argument: Option<String>,
    environment: impl FnOnce() -> Result<String, env::VarError>,
) -> Result<Option<Selection>, CliError> {
    let requested = match argument {
        Some(value) => Some(value),
        None => match environment() {
            Ok(value) => Some(value),
            Err(env::VarError::NotPresent) => None,
            Err(err) => return Err(failure(format!("invalid RFEL_DEVICE: {err}"))),
        },
    };
    requested
        .map(|requested| {
            let index = match_selector(selectors, &requested)?.ok_or_else(|| {
                failure(format!(
                    "no device matches {requested:?}; run `rfel version` to list selectors"
                ))
            })?;
            Ok(Selection {
                index,
                save_path: None,
            })
        })
        .transpose()
}

fn interactive_selection(
    selectors: &[String],
    index: Option<usize>,
    path: PathBuf,
) -> Result<Selection, CliError> {
    match index {
        Some(index) if index < selectors.len() => {
            match_selector(selectors, &selectors[index])?;
            Ok(Selection {
                index,
                save_path: Some(path),
            })
        }
        _ => Err(failure(
            "device selection cancelled; no command executed or configuration saved",
        )),
    }
}

impl Selection {
    /// Persist only after the chosen connection has opened and its chip was detected.
    pub(super) fn save(&self, info: &DeviceInfo) -> Result<(), CliError> {
        if let Some(path) = &self.save_path {
            save_selector(path, &selector(info))?;
            eprintln!("Selected {}; saved to {}", selector(info), path.display());
        }
        Ok(())
    }
}

fn save_selector(path: &Path, selector: &str) -> Result<(), CliError> {
    let mut config = read_config(path)?;
    config["device"] = value(selector);
    let write = || -> std::io::Result<()> {
        let mut temporary =
            tempfile::NamedTempFile::new_in(path.parent().unwrap_or(Path::new(".")))?;
        temporary.write_all(config.to_string().as_bytes())?;
        temporary.as_file().sync_all()?;
        temporary.persist(path).map_err(|err| err.error)?;
        Ok(())
    };
    write().map_err(|err| failure(format!("cannot save {}: {err}", path.display())))
}

pub(super) fn select(
    devices: &[DeviceInfo],
    argument: Option<String>,
) -> Result<Selection, CliError> {
    let selectors: Vec<_> = devices.iter().map(selector).collect();
    // Explicit selection bypasses config IO, including a malformed config file.
    if let Some(selection) = explicit_selection(&selectors, argument, || env::var("RFEL_DEVICE"))? {
        return Ok(selection);
    }

    let cwd = env::current_dir()
        .map_err(|err| failure(format!("cannot get working directory: {err}")))?;
    let path = config_path(&cwd);
    let config = read_config(&path)?;
    let requested = configured_selector(&config)?;
    if let Some(requested) = requested {
        if let Some(index) = match_selector(&selectors, requested)? {
            return Ok(Selection {
                index,
                save_path: None,
            });
        }
        eprintln!(
            "Saved device {requested:?} from {} is not connected.",
            path.display()
        );
    } else if devices.len() == 1 {
        return Ok(Selection {
            index: 0,
            save_path: None,
        });
    }

    if !std::io::stdin().is_terminal() || !std::io::stderr().is_terminal() {
        return Err(failure(
            "device selection requires an interactive terminal; use --device or RFEL_DEVICE (see `rfel version`)",
        ));
    }
    let _screen = SelectionScreen::enter()?;
    eprintln!("RFEL - SELECT A DEVICE\n");
    eprintln!("Action required: choose a device to continue.");
    eprintln!("Up/Down: select   Enter: use and save   Esc/q: exit without saving\n");
    let width = dialoguer::console::Term::stderr()
        .size()
        .1
        .saturating_sub(4) as usize;
    let mut items: Vec<_> = devices
        .iter()
        .enumerate()
        .map(|(index, info)| {
            let version = match version(info) {
                Ok(version) => format!(
                    "{} id={:08x}",
                    version
                        .chip()
                        .map(|chip| format!("{chip:?}"))
                        .unwrap_or_else(|| "Unknown".to_string()),
                    version.id()
                ),
                Err(err) => format!("error: {err}"),
            };
            let row = format!(
                "{}: {version} | addr={} ports={:?} path={}",
                index + 1,
                info.device_address(),
                info.port_chain(),
                selector(info)
            );
            dialoguer::console::truncate_str(&row, width, "...").into_owned()
        })
        .collect();
    items.push("Exit without saving or executing".to_string());
    eprintln!(
        "Enter saves the selected USB port to {}. Up/Down selects; Esc/q exits.",
        path.display()
    );
    let index = Select::new()
        .with_prompt("Select FEL device")
        .items(&items)
        .default(0)
        .max_length(8)
        .interact_opt()
        .map_err(|err| failure(format!("device selection failed: {err}")))?;
    interactive_selection(&selectors, index, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_selection_never_saves_and_argument_overrides_environment() {
        let selectors = vec!["1-2".into(), "1-3".into()];
        let selection = explicit_selection(&selectors, Some(selectors[0].clone()), || {
            panic!("environment must not be read")
        })
        .unwrap()
        .unwrap();
        assert_eq!(selection.index, 0);
        assert!(selection.save_path.is_none());
        let selection = explicit_selection(&selectors, None, || Ok(selectors[1].clone()))
            .unwrap()
            .unwrap();
        assert_eq!(selection.index, 1);
        assert!(selection.save_path.is_none());
        assert!(
            explicit_selection(&selectors, Some("missing".into()), || Ok(
                selectors[1].clone()
            ))
            .is_err()
        );
    }

    #[test]
    fn only_confirmed_interaction_requests_persistence() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join(CONFIG_NAME);
        let selectors = vec!["1-2".into(), "1-3".into()];
        for index in [None, Some(2)] {
            assert!(interactive_selection(&selectors, index, path.clone()).is_err());
            assert!(!path.exists());
        }
        fs::write(&path, "device = 'previous'\n").unwrap();
        assert!(interactive_selection(&selectors, None, path.clone()).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "device = 'previous'\n");
        let selection = interactive_selection(&selectors, Some(1), path.clone()).unwrap();
        assert_eq!(selection.index, 1);
        assert_eq!(selection.save_path, Some(path));
    }

    #[test]
    fn topology_follows_dfu_util_and_windows_location_paths() {
        assert_eq!(topology_selector("1", &[2, 3]), "1-2.3");
        assert_eq!(topology_selector("001", &[2, 3]), "1-2.3");
        assert_eq!(
            topology_selector("PCIROOT(0)#PCI(1400)#USBROOT(0)", &[2, 3]),
            "PCIROOT(0)#PCI(1400)#USBROOT(0)#USB(2)#USB(3)"
        );
    }

    #[test]
    fn matching_never_falls_back_or_picks_a_duplicate() {
        let selectors = vec!["1-2".into(), "1-3".into()];
        assert_eq!(match_selector(&selectors, "1-3").unwrap(), Some(1));
        assert_eq!(match_selector(&selectors, "1-4").unwrap(), None);
        assert!(match_selector(&["same".into(), "same".into()], "same").is_err());
    }

    #[test]
    fn config_search_stops_at_repository_and_prefers_nearest_file() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("repo");
        let child = root.join("example");
        fs::create_dir_all(&child).unwrap();
        fs::write(root.join(".git"), "gitdir: elsewhere").unwrap();
        fs::write(temp.path().join(CONFIG_NAME), "device = 'outside'").unwrap();
        assert_eq!(config_path(&child), root.join(CONFIG_NAME));
        fs::write(child.join(CONFIG_NAME), "device = 'local'").unwrap();
        assert_eq!(config_path(&child), child.join(CONFIG_NAME));
    }

    #[test]
    fn config_round_trips_and_preserves_other_settings() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join(CONFIG_NAME);
        fs::write(&path, "# User comment\nother = 42\ndevice = 'old'\n").unwrap();
        let selector = topology_selector("PCIROOT(0)#PCI(1400)#USBROOT(0)", &[2]);
        save_selector(&path, &selector).unwrap();
        let config = read_config(&path).unwrap();
        assert_eq!(
            configured_selector(&config).unwrap(),
            Some(selector.as_str())
        );
        assert_eq!(config["other"].as_integer(), Some(42));
        assert!(fs::read_to_string(path).unwrap().contains("# User comment"));
    }

    #[test]
    fn malformed_config_is_not_overwritten() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join(CONFIG_NAME);
        fs::write(&path, "not = [valid").unwrap();
        assert!(save_selector(&path, "1-2").is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "not = [valid");
        let config = "device = 2".parse().unwrap();
        assert!(configured_selector(&config).is_err());
    }

    #[test]
    fn device_option_is_global_and_version_remains_a_command() {
        use clap::Parser;
        for args in [
            vec!["rfel", "--device", "1-2", "version"],
            vec!["rfel", "version", "--device", "1-2"],
            vec![
                "rfel", "flash", "read", "0", "4", "out.bin", "--device", "1-2",
            ],
        ] {
            let cli = super::super::Cli::try_parse_from(args).unwrap();
            assert_eq!(cli.device.as_deref(), Some("1-2"));
        }
    }
}
