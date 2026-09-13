//! Allocation-free command-line editing for the serial console examples.

/// A completed console command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    Empty,
    Help,
    Hello,
    Exit,
    Unknown,
}

/// The visible action caused by one received byte.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputEvent {
    None,
    Echo(u8),
    Erase,
    Bell,
    Command(Command),
}

/// Minimal line editor used by the CDC-ACM console.
pub struct Console<const N: usize> {
    line: [u8; N],
    len: usize,
    swallow_lf: bool,
}

impl<const N: usize> Console<N> {
    pub const fn new() -> Self {
        Self {
            line: [0; N],
            len: 0,
            swallow_lf: false,
        }
    }

    pub fn push(&mut self, byte: u8) -> InputEvent {
        if byte == b'\n' && self.swallow_lf {
            self.swallow_lf = false;
            return InputEvent::None;
        }
        self.swallow_lf = false;

        match byte {
            b'\r' | b'\n' => {
                self.swallow_lf = byte == b'\r';
                let command = classify(&self.line[..self.len]);
                self.len = 0;
                InputEvent::Command(command)
            }
            0x08 | 0x7f if self.len != 0 => {
                self.len -= 1;
                InputEvent::Erase
            }
            0x08 | 0x7f => InputEvent::None,
            0x20..=0x7e if self.len < N => {
                self.line[self.len] = byte;
                self.len += 1;
                InputEvent::Echo(byte)
            }
            0x20..=0x7e => InputEvent::Bell,
            _ => InputEvent::None,
        }
    }
}

impl<const N: usize> Default for Console<N> {
    fn default() -> Self {
        Self::new()
    }
}

fn classify(mut line: &[u8]) -> Command {
    while let Some((b' ', rest)) = line.split_first() {
        line = rest;
    }
    while let Some((b' ', rest)) = line.split_last() {
        line = rest;
    }

    match line {
        b"" => Command::Empty,
        b"help" => Command::Help,
        b"hello" => Command::Hello,
        b"exit" => Command::Exit,
        _ => Command::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_commands_and_trims_spaces() {
        assert_eq!(classify(b"help"), Command::Help);
        assert_eq!(classify(b"  hello  "), Command::Hello);
        assert_eq!(classify(b"exit"), Command::Exit);
        assert_eq!(classify(b"HELLO"), Command::Unknown);
    }

    #[test]
    fn treats_crlf_as_one_line_ending() {
        let mut console = Console::<8>::new();
        for byte in b"help" {
            assert_eq!(console.push(*byte), InputEvent::Echo(*byte));
        }
        assert_eq!(console.push(b'\r'), InputEvent::Command(Command::Help));
        assert_eq!(console.push(b'\n'), InputEvent::None);
    }

    #[test]
    fn supports_backspace_without_underflow() {
        let mut console = Console::<2>::new();
        assert_eq!(console.push(0x08), InputEvent::None);
        assert_eq!(console.push(b'x'), InputEvent::Echo(b'x'));
        assert_eq!(console.push(0x7f), InputEvent::Erase);
        assert_eq!(console.push(b'\r'), InputEvent::Command(Command::Empty));
    }
}
