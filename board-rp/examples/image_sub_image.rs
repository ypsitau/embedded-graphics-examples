#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
}

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::image_sub_image as example;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg0);
    let mut board = board_rp::init_board_with_mipidsi::<0, _>(mipidsi::models::ST7789, orientation);
    example::show(&mut board.display).unwrap();
    core::future::pending::<()>().await;
}
