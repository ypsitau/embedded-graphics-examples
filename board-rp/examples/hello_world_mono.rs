#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
}
mod common;

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::hello_world_mono as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    use common::Flushable;
    let rotation = ssd1306::rotation::DisplayRotation::Rotate0;
    let mut board = common::init_board_with_ssd1306::<0>(rotation);
    example::show(&mut board.display).unwrap();
    board.display.flush().unwrap();
    core::future::pending::<()>().await;
}
