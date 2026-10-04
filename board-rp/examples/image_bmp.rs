#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
}
mod common;

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::image_bmp as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg0);
    let (mut display, _pin_display_bl, mut _i2c_dev) = common::init_board(mipidsi::models::ST7789, orientation);
    example::show(&mut display).unwrap();
    core::future::pending::<()>().await;
}
