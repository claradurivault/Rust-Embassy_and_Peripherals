#![no_main]
#![no_std]

use embassy_stm32::gpio::AnyPin;
use embassy_stm32::{Peri, Peripherals};
pub struct Board {
    pub bargraph_pins: Bargraph_Pins,
    pub gamepad_pins: Gamepad_Pins,
}

impl Board {
    pub fn new(p: Peripherals) -> Self {
        Self {
            bargraph_pins: Bargraph_Pins {
                led0: p.PC7.into(),
                led1: p.PB2.into(),
                led2: p.PA8.into(),
                led3: p.PB1.into(),
                led4: p.PB15.into(),
                led5: p.PB4.into(),
                led6: p.PB14.into(),
                led7: p.PB5.into(),
            },
        }
    }
}

pub struct Bargraph_Pins {
    pub led0: Peri<'static, AnyPin>,
    pub led1: Peri<'static, AnyPin>,
    pub led2: Peri<'static, AnyPin>,
    pub led3: Peri<'static, AnyPin>,
    pub led4: Peri<'static, AnyPin>,
    pub led5: Peri<'static, AnyPin>,
    pub led6: Peri<'static, AnyPin>,
    pub led7: Peri<'static, AnyPin>,
}