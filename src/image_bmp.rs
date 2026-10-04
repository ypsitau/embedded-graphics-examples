//! # Example: BMP images
//!
//! Draw a 16BPP BMP image of the Rust logo to a display
//!
//! This example uses the [tinybmp](https://crates.io/crates/tinybmp) crate to load the BMP from a
//! byte slice read in at compile time.
//!
//! The `graphics` feature of `tinybmp` needs to be enabled in `Cargo.toml` to use the `Bmp` object
//! with embedded-graphics.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor
        + From<eg::pixelcolor::Rgb555> + From<eg::pixelcolor::Rgb565> + From<eg::pixelcolor::Rgb888>,
{
    target.clear(ColorType::BLACK)?;
    let center = target.bounding_box().center();
    let bmp = tinybmp::Bmp::<ColorType>::from_slice(include_bytes!("./assets/rust-pride.bmp")).unwrap();
    eg::image::Image::with_center(&bmp, center).draw(target)?;
    Ok(())
}
