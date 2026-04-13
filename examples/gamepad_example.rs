#![no_main]
#![no_std]

use defmt::info;
use tp3::drivers::gamepad::Gamepad;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hardware initialized, starting main loop...");
    let gamepad = Gamepad::new(
            p.PC8.into(),
            p.PB11.into(),
            p.PC6.into(),
            p.PC9.into(),
            p.PC5.into(),
        );
    loop {
        let state = gamepad.poll();
        info!("Gamepad state: {}", state.display());
        Timer::after_millis(500).await;
    }
}