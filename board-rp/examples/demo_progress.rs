#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::demo_progress as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    use mipidsi::options as opts;
    let mut board = board_rp::init_with_mipidsi::<0, _>(mipidsi::models::ST7789, (240, 320),
        opts::Orientation::new().rotate(opts::Rotation::Deg0),
        opts::ColorOrder::Rgb, opts::ColorInversion::Inverted);
    example::run(&mut board.display, emb::time::Delay, |_| { false }).await.unwrap();
}
