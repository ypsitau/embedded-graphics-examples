#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::demo_progress_mono as example;
use board_rp::prelude::*;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let rotation = ssd1306::rotation::DisplayRotation::Rotate0;
    let mut board = board_rp::init_with_ssd1306::<0>(rotation);
    example::run(&mut board.display, emb::time::Delay, |display| {
        display.flush().unwrap();
        false
    }).await.unwrap();
}
