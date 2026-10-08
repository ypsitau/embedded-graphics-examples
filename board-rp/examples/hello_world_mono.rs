#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
}

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::hello_world_mono as example;
use board_rp::Flushable as _;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let rotation = ssd1306::rotation::DisplayRotation::Rotate0;
    let mut board = board_rp::init_board_with_ssd1306::<0>(rotation);
    example::show(&mut board.display).unwrap();
    board.display.flush().unwrap();
    core::future::pending::<()>().await;
}
