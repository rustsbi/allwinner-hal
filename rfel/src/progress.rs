use std::io::{self, Write};
use std::time::{Duration, Instant};

/// Lightweight terminal progress bar.
pub struct Progress {
    label: &'static str,
    total: u64,
    done: u64,
    start: Instant,
    last: Instant,
    tick_every: Duration,
}

impl Progress {
    pub fn new(label: &'static str, total: u64) -> Self {
        let now = Instant::now();
        Self {
            label,
            total,
            done: 0,
            start: now,
            last: now,
            tick_every: Duration::from_millis(100),
        }
    }

    pub fn inc(&mut self, n: u64) {
        self.done = self.done.saturating_add(n);
        let now = Instant::now();
        if now.duration_since(self.last) >= self.tick_every || self.done >= self.total {
            self.last = now;
            self.draw(now);
        }
    }

    pub fn finish(&mut self) {
        self.draw(Instant::now());
        println!();
    }

    fn draw(&self, now: Instant) {
        let elapsed = now.duration_since(self.start).as_secs_f64().max(1e-6);
        let speed = self.done as f64 / elapsed; // B/s
        let (spd_val, spd_unit) = human_speed(speed);
        let pct = if self.total > 0 {
            (self.done as f64 / self.total as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        let eta_secs = if speed > 0.0 && self.total > self.done {
            ((self.total - self.done) as f64 / speed) as u64
        } else {
            0
        };
        let bar = render_bar(pct, 30);
        print!(
            "\r{:<5} {} {:6.2}% {:5.1} {}/s ETA {} {}/{}",
            self.label,
            bar,
            pct,
            spd_val,
            spd_unit,
            fmt_eta(eta_secs),
            human_bytes(self.done),
            human_bytes(self.total),
        );
        let _ = io::stdout().flush();
    }
}

fn render_bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let mut s = String::with_capacity(width + 2);
    s.push('[');
    for i in 0..width {
        s.push(if i < filled { '#' } else { ' ' });
    }
    s.push(']');
    s
}

fn human_bytes(n: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut val = n as f64;
    let mut idx = 0usize;
    while val >= 1024.0 && idx + 1 < UNITS.len() {
        val /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{:.0}{}", val, UNITS[idx])
    } else {
        format!("{:.2}{}", val, UNITS[idx])
    }
}

fn human_speed(bps: f64) -> (f64, &'static str) {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut val = bps;
    let mut idx = 0usize;
    while val >= 1024.0 && idx + 1 < UNITS.len() {
        val /= 1024.0;
        idx += 1;
    }
    (val, UNITS[idx])
}

fn fmt_eta(mut secs: u64) -> String {
    let h = secs / 3600;
    secs %= 3600;
    let m = secs / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_bytes() {
        assert_eq!(human_bytes(0), "0B");
        assert_eq!(human_bytes(1023), "1023B");
        assert_eq!(human_bytes(1024), "1.00KiB");
        assert_eq!(human_bytes(1024 * 1024), "1.00MiB");
    }

    #[test]
    fn test_fmt_eta() {
        assert_eq!(fmt_eta(59), "00:59");
        assert_eq!(fmt_eta(61), "01:01");
        assert_eq!(fmt_eta(3661), "01:01:01");
    }

    #[test]
    fn test_human_speed() {
        let (v, u) = human_speed(500.0);
        assert_eq!(u, "B");
        assert!((v - 500.0).abs() < 1e-9);

        let (v, u) = human_speed(1024.0);
        assert_eq!(u, "KiB");
        assert!((v - 1.0).abs() < 1e-9);

        let (v, u) = human_speed(1536.0);
        assert_eq!(u, "KiB");
        assert!((v - 1.5).abs() < 1e-9);
    }

    #[test]
    fn test_render_bar() {
        let b0 = render_bar(0.0, 10);
        assert_eq!(b0, "[          ]");
        let b50 = render_bar(50.0, 10);
        assert_eq!(b50, "[#####     ]");
        let b100 = render_bar(100.0, 10);
        assert_eq!(b100, "[##########]");
    }
}
