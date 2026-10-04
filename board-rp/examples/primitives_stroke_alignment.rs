#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}
mod common;

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::primitives_stroke_alignment as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg0);
    let (mut display, _pin_display_bl, mut _i2c_dev) = common::init_board(mipidsi::models::ST7789, orientation);
    example::run(&mut display, emb::time::Delay, |_| -> example::Event { example::Event::None } ).unwrap();
}
