use embedded_time::rate::Baud;

/// Serial configuration structure.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    /// Serial baudrate in `Bps`.
    pub baudrate: Baud,
    /// Word length, can be 5, 6, 7 or 8.
    pub wordlength: WordLength,
    /// Parity checks, can be `None`, `Odd` or `Even`.
    pub parity: Parity,
    /// Number of stop bits, can be `One` or `Two`.
    pub stopbits: StopBits,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        use embedded_time::rate::Extensions;
        Self {
            baudrate: 115200.Bd(),
            wordlength: WordLength::Eight,
            parity: Parity::None,
            stopbits: StopBits::One,
        }
    }
}

/// Serial word length settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WordLength {
    /// 5 bits per word.
    Five,
    /// 6 bits per word.
    Six,
    /// 7 bits per word.
    Seven,
    /// 8 bits per word.
    Eight,
}

/// Serial parity bit settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Parity {
    /// No parity checks.
    None,
    /// Odd parity.
    Odd,
    /// Even parity.
    Even,
}

/// Stop bit settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StopBits {
    /// 1 stop bit
    One,
    /// 2 stop bits, or 1.5 bits when WordLength is Five
    Two,
}
