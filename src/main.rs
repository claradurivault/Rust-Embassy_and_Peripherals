#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_stm32::Config;
use defmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    info!("Carte initialisée !");
    loop {}
}
