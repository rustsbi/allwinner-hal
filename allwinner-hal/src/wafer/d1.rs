//! SoC configuration on D1-like chips.

use crate::{smhc, spi};

// SPI PINS
impl_pins_trait! {
    ('B', 9, 5): spi::IntoMiso<1>;
    ('B', 10, 5): spi::IntoMosi<1>;
    ('B', 11, 5): spi::IntoClk<1>;
    ('C', 2, 2): spi::IntoClk<0>;
    ('C', 4, 2): spi::IntoMosi<0>;
    ('C', 5, 2): spi::IntoMiso<0>;
    ('D', 11, 4): spi::IntoClk<1>;
    ('D', 12, 4): spi::IntoMosi<1>;
    ('D', 13, 4): spi::IntoMiso<1>;
}

// SMHC pins
impl_pins_trait! {
    ('F', 0, 2): smhc::Data<1>;
    ('F', 1, 2): smhc::Data<0>;
    ('F', 2, 2): smhc::Clk;
    ('F', 3, 2): smhc::Cmd;
    ('F', 4, 2): smhc::Data<3>;
    ('F', 5, 2): smhc::Data<2>;
    ('G', 0, 2): smhc::Clk;
    ('G', 1, 2): smhc::Cmd;
    ('G', 2, 2): smhc::Data<0>;
    ('G', 3, 2): smhc::Data<1>;
    ('G', 4, 2): smhc::Data<2>;
    ('G', 5, 2): smhc::Data<3>;
    ('C', 2, 3): smhc::Clk;
    ('C', 3, 3): smhc::Cmd;
    ('C', 4, 3): smhc::Data<2>;
    ('C', 5, 3): smhc::Data<1>;
    ('C', 6, 3): smhc::Data<0>;
    ('C', 7, 3): smhc::Data<3>;
}
