#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}
mod ds3231;

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::demo_analog_clock as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg0);
    let mut board = board_rp::init_board_with_mipidsi::<1, _>(mipidsi::models::ST7789, orientation);
    let mut rtc = ds3231::Ds3231::new(&mut board.i2c_devs[0]);
    let fn_get_time = || {
        let (hour, minute, second) = rtc.read();
        (hour, minute, second)
    };
    example::run(&mut board.display, emb::time::Delay, fn_get_time, |_| { false }).await.unwrap();
}
