#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_time as time;
}

use {defmt_rtt as _, embedded_graphics::draw_target::DrawTarget, panic_probe as _};
use embedded_graphics_examples::demo_pacman as example;
use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_framebuf as egfb;
use mipidsi::models::ST7789 as DisplayModel;
use static_cell::StaticCell;
type ColorType = eg::pixelcolor::Rgb565;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    use mipidsi::options as opts;
    let mut board = board_rp::init_with_mipidsi::<0, _>(DisplayModel, (240, 320),
        opts::Orientation::new().rotate(opts::Rotation::Deg0),
        opts::ColorOrder::Rgb, opts::ColorInversion::Inverted);
    let mut framebuf = {
        use mipidsi::models::Model as _;
        const DISPLAY_SIZE: (u16, u16) = DisplayModel::FRAMEBUFFER_SIZE;
        const BUF_SIZE: usize = DISPLAY_SIZE.0 as usize * DISPLAY_SIZE.1 as usize;
        static STATIC_CELL: StaticCell<[ColorType; BUF_SIZE]> = StaticCell::new();
        let buf = STATIC_CELL.init([ColorType::BLACK; BUF_SIZE]);
        egfb::FrameBuf::new(buf, DISPLAY_SIZE.0 as usize, DISPLAY_SIZE.1 as usize)
    };
    //let mut framebuf = create_framebuf::<DisplayModel>();
    example::run(&mut framebuf, emb::time::Delay,
        |framebuf| {
            let area = eg::primitives::Rectangle::new(Point::zero(), framebuf.size());
            board.display.fill_contiguous(&area, framebuf.data.iter().copied()).unwrap();
            false
        }).await.unwrap();
}
