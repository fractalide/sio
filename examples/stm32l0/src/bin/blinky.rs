#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::Peripherals;
use {defmt_rtt as _, panic_probe as _};
use sio_vm::{tick};

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let mut led = Output::new(p.PB5, Level::High, Speed::Low);

    loop {
        info!("{}", tick().await);

        info!("led on!");
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;

        info!("led off!");
        led.set_low();
        Timer::after(Duration::from_millis(300)).await;
    }
}
