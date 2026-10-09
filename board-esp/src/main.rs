#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
}

use esp_backtrace as _;
use embedded_graphics_examples::hello_world as example;

#[esp_rtos::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg90);
    let mut board = board_esp::init_with_mipidsi::<_>(mipidsi::models::ST7789, orientation);
    example::show(&mut board.display).unwrap();
    core::future::pending::<()>().await;
}
