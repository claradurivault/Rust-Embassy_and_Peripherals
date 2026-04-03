#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Input, Speed, AnyPin, Output, Pull};
use embassy_stm32::Peri;
use embassy_time::Timer;
use embedded_hal::digital::v2::OutputPin;
use {defmt_rtt as _, panic_probe as _};

/*pub struct Gamepad<const N: usize> {
    buttons: [Input<'static>; N],
}

impl<const N: usize> Gamepad<N> {
    fn new(buttons: [Input<'static>; N]) -> Self {
        let buttons = raw.map(|pin| Input::new(pin, Pull::Down));
        Self { buttons }
    }

}*/