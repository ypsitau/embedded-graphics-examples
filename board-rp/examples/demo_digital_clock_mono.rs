#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}
mod ds3231;
    
use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::demo_digital_clock_mono as example;
use board_rp::Flushable as _;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let rotation = ssd1306::rotation::DisplayRotation::Rotate0;
    let mut board = board_rp::init_with_ssd1306::<1>(rotation);
    let mut rtc = ds3231::Ds3231::new(&mut board.i2c_devs[0]);
    let fn_get_time = || {
        let (hour, minute, second) = rtc.read();
        (hour, minute, second)
    };
    example::run(&mut board.display, emb::time::Delay, fn_get_time, |display| {
        display.flush().unwrap();
        false
    }).await.unwrap();
}
