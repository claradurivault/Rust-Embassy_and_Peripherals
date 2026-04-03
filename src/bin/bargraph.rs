#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed, AnyPin};
use embassy_stm32::Peri;
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
        #[panic_handler]
        for led in &mut self.leds {
            led.set_high();
        }
    }

    pub fn all_low(&mut self) {
        #[panic_handler]
        for led in &mut self.leds {
            led.set_low();
        }
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

    bargraph_pins.all_high();
}