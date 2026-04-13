#![no_main]
#![no_std]

use defmt::info;
use tp3::drivers::bargraph::Bargraph;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("Hardware initialized, starting main loop...");

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