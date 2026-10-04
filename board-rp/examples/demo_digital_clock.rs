#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}
mod common;
mod ds3231;

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::demo_digital_clock as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg90);
    let (mut display, _pin_display_bl, mut i2c_dev) = common::init_board(mipidsi::models::ST7789, orientation);
    let mut rtc = ds3231::Ds3231::new(&mut i2c_dev);
    let func_get_time = || -> example::Time {
        let (hour, minute, second) = rtc.read();
        example::Time::new(hour, minute, second)
    };
    example::run(&mut display, emb::time::Delay, func_get_time, |_| -> bool { false }).unwrap();
}
