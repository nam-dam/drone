#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    adc::Adc,
    gpio::{Input, Pull},
};
use embassy_time::Delay;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Started controller");

    let mut delay = Delay;
    let mut adc = Adc::new(p.ADC1, &mut delay);

    let button = Input::new(p.PB0, Pull::Up);

    let mut pin0 = p.PA0;
    let mut pin1 = p.PA1;

    loop {
        let xPos = adc.read(&mut pin0);
        let yPos = adc.read(&mut pin1);

        info!("xPos {} yPos {}", xPos, yPos);

        if button.is_high() {
            info!("released");
        } else {
            info!("pressed");
        }
    }
}

// #![no_std]
// #![no_main]
// #![feature(type_alias_impl_trait)]

// use defmt::*;
// use embassy_executor::Spawner;
// use embassy_stm32::gpio::{Level, Output, Speed};
// use embassy_time::{Duration, Timer};
// use {defmt_rtt as _, panic_probe as _};

// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     let p = embassy_stm32::init(Default::default());
//     info!("Hello World!");

//     let mut led = Output::new(p.PC13, Level::High, Speed::Low);

//     loop {
//         info!("high");
//         led.set_high();
//         Timer::after(Duration::from_millis(300)).await;

//         info!("low");
//         led.set_low();
//         Timer::after(Duration::from_millis(300)).await;
//     }
// }
