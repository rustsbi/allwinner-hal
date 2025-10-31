use clap::Parser;
use rfel::cli;
use std::error::Error;

fn main() {
    let cli = cli::Cli::parse();
    if let Err(err) = cli::run(cli) {
        eprintln!("{}", err);
        if let Some(source) = err.source() {
            eprintln!("caused by: {}", source);
        }
        std::process::exit(1);
    }
}
