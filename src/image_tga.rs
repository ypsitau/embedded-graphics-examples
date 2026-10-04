//! # Example: TGA images
//!
//! Draw a 24BPP TGA image to the display
//!
//! This example uses the [tinytga](https://crates.io/crates/tinytga) crate to load the TGA from a
//! byte slice read in at compile time.
//!
//! The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
//! with embedded-graphics.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor
        + From<eg::pixelcolor::Gray8> + From<eg::pixelcolor::Rgb555>
        + From<eg::pixelcolor::Rgb565> + From<eg::pixelcolor::Rgb888>,
{
    target.clear(ColorType::BLACK)?;
    let center = target.bounding_box().center();
    let tga = tinytga::Tga::<ColorType>::from_slice(include_bytes!("./assets/rust-pride.tga")).unwrap();
    eg::image::Image::with_center(&tga, center).draw(target)?;
    Ok(())
}
