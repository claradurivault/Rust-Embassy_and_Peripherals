use embassy_stm32::gpio::{Input, AnyPin, Pull};
use embassy_stm32::Peri;
use {defmt_rtt as _, panic_probe as _};

pub struct Button {
    pub button: Input<'static>
}

impl Button {
    pub fn new(pin: Peri<'static, AnyPin>) -> Self {
        let button = Input::new(pin, Pull::Down);
        Self { button }
    }

    pub fn is_pressed(&self) -> bool {
        self.button.is_low()
    }
}
