#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
}

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::primitives_triangles as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    use mipidsi::options as opts;
    let mut board = board_rp::init_with_mipidsi::<0, _>(mipidsi::models::ST7789, (240, 320),
        opts::Orientation::new().rotate(opts::Rotation::Deg90),
        opts::ColorOrder::Rgb, opts::ColorInversion::Inverted);
    example::show(&mut board.display).unwrap();
    core::future::pending::<()>().await;
}
