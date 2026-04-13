use embassy_stm32::gpio::{Level, Output, Speed, AnyPin};
use embassy_stm32::Peri;
use {defmt_rtt as _, panic_probe as _};

pub struct Bargraph<const N: usize> {
    leds: [Output<'static>; N],
}

impl<const N: usize> Bargraph<N> {
    pub fn new(leds: [Peri<'static, AnyPin>; N]) -> Self {
        let leds = leds.map(|pin| Output::new(pin, Level::Low, Speed::Low));
        Self { leds }
    }

    pub fn all_high(&mut self) {
        for led in &mut self.leds {
            led.set_high();
        }
    }

    pub fn all_low(&mut self) {
        for led in &mut self.leds {
            led.set_low();
        }
    }
    pub fn set_range(&mut self, start : usize, end: usize) {
        for i in 0..N {
            if i >= start && i < end {
                self.leds[i].set_high();
            } else {
                self.leds[i].set_low();
            }
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.set_range(0, value);
    }

}