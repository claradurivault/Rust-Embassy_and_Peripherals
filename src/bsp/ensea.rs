use crate::drivers::bargraph::Bargraph;
use crate::drivers::gamepad::Gamepad;

use embassy_stm32::{Peripherals};
pub struct Board {
    pub bargraph: Bargraph<7>,
    pub gamepad: Gamepad,
}

impl Board {
    pub fn new(p: Peripherals) -> Self {
        // Bargraph pins
        let bargraph_pins = [
            p.PC7.into(),
            p.PB2.into(),
            p.PA8.into(),
            p.PB1.into(),
            p.PB15.into(),
            p.PB4.into(),
            p.PB14.into(),
        ];
        let bargraph = Bargraph::new(bargraph_pins);
        // Gamepad pins
        let gamepad = Gamepad::new(
            p.PC8.into(),
            p.PB11.into(),
            p.PC6.into(),
            p.PC9.into(),
            p.PC5.into(),
        );
        Self { bargraph, gamepad }
    }
}