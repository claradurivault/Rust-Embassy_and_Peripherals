#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed, AnyPin};
use embassy_stm32::Peri;
use embassy_time::Timer;
use embedded_hal::digital::v2::OutputPin;
use {defmt_rtt as _, panic_probe as _};

pub struct Bargraph<const N: usize> {
    leds: [Output<'static>; N],
}

impl<const N: usize> Bargraph<N> {
    pub fn new(raw: [Peri<'static, AnyPin>; N]) -> Self {
        let leds = raw.map(|pin| Output::new(pin, Level::Low, Speed::Low));
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


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let p = embassy_stm32::init(Default::default());
    let tab = [
        p.PC7.into(),
        p.PB2.into(),
        p.PA8.into(),
        p.PB1.into(),
        p.PB15.into(),
        p.PB4.into(),
        p.PB14.into(),
        p.PB5.into()];
    let mut bargraph_pins = Bargraph::<8>::new(tab);

    loop {
        // chenillard
        for i in 0..8 {
            bargraph_pins.set_value(i);
            Timer::after_millis(1_000).await;
        }
        Timer::after_millis(1_000).await;
        bargraph_pins.all_low();
        Timer::after_millis(1_000).await;
    }
}