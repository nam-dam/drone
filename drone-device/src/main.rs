#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    dma::NoDma,
    i2c,
    i2c::{I2c, TimeoutI2c},
    peripherals,
    time::Hertz,
};
use embassy_time::{Delay, Duration};
use mpu6050::Mpu6050;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0x5F;
const WHOAMI: u8 = 0x0F;

bind_interrupts!(struct Irqs {
    I2C2_EV => i2c::InterruptHandler<peripherals::I2C2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Started device");

    let mut delay = Delay;
    let mut i2c = I2c::new(
        p.I2C2,
        p.PB10,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    // I2C bus can freeze if SCL line is shorted or due to a broken device that clock stretches for too long.
    // TimeoutI2c allows recovering from such errors by throwing `Error::Timeout` after a given delay.
    let timeout_i2c = TimeoutI2c::new(&mut i2c, Duration::from_millis(1000));

    let mut mpu = Mpu6050::new(timeout_i2c);

    let _ = mpu.init(&mut delay);

    loop {
        // get roll and pitch estimate
        let angle = mpu.get_acc_angles().ok().unwrap();
        println!("r/p: {:?} {:?}", angle.x, angle.y);

        // get temp
        let temp = mpu.get_temp().ok().unwrap();
        println!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().ok().unwrap();
        println!("gyro: {:?} {:?}", gyro.x, gyro.y);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().ok().unwrap();
        println!("acc: {:?} {:?} {:?}", acc.x, acc.y, acc.z);
    }
}
