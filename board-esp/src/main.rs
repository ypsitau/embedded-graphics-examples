#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}

use esp_backtrace as _;
use embedded_graphics_examples::hello_world as example;
//use embedded_graphics_examples::demo_progress as example;

#[esp_rtos::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg90);
    let mut board = board_esp::init_with_mipidsi::<0, _>(mipidsi::models::ST7789, (240, 320),
        orientation, mipidsi::options::ColorOrder::Rgb, mipidsi::options::ColorInversion::Inverted);
    //example::run(&mut board.display, emb::time::Delay, |_| { false }).await.unwrap();
    example::show(&mut board.display).unwrap();
    core::future::pending::<()>().await;
}
