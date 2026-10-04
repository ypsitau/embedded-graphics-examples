#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}
use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples as examples;

mod common;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new()
        .rotate(mipidsi::options::Rotation::Deg90).flip_vertical();
    let (mut display, _pin_display_bl) = common::init_display(mipidsi::models::ST7789, orientation);
    examples::primitives_fill::show(&mut display).unwrap();
    //examples::demo_pacman::run(&mut display, emb::time::Delay, |_| -> bool { false }).unwrap();
    core::future::pending::<()>().await;
}
