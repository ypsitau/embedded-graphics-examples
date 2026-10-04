//! # Example: Sub images
//!
//! Loads a TGA file using the [tinytga](https://crates.io/crates/tinytga) crate and splits it
//! into multiple sub images.
//!
//! The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
//! with embedded-graphics.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii as eg_fonts;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor
        + From<eg::pixelcolor::Gray8> + From<eg::pixelcolor::Rgb555>
        + From<eg::pixelcolor::Rgb565> + From<eg::pixelcolor::Rgb888>,
{
    target.clear(ColorType::BLACK)?;
    // Load TGA file with the tiles.
    let tiles = tinytga::Tga::<ColorType>::from_slice(include_bytes!("./assets/tiles.tga")).unwrap();

    // Create sub images for the individual tiles.
    // Note that the tiles don't have to be the same size.
    let tile_a = tiles.sub_image(&eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(64, 64)));
    let tile_b = tiles.sub_image(&eg::primitives::Rectangle::new(Point::new(64, 0), Size::new(64, 64)));
    let tile_c = tiles.sub_image(&eg::primitives::Rectangle::new(Point::new(0, 64), Size::new(128, 64)));

    let text_style = eg::mono_font::MonoTextStyle::new(&eg_fonts::FONT_6X13, ColorType::WHITE);

    eg::text::Text::new("TGA image", Point::new(10, 70), text_style).draw(target)?;
    eg::image::Image::new(&tiles, Point::new(100, 10)).draw(target)?;

    eg::text::Text::new("Tile A", Point::new(10, 190), text_style).draw(target)?;
    eg::image::Image::new(&tile_a, Point::new(50, 160)).draw(target)?;

    eg::text::Text::new("Tile B", Point::new(120, 190), text_style).draw(target)?;
    eg::image::Image::new(&tile_b, Point::new(160, 160)).draw(target)?;

    eg::text::Text::new("Tile C", Point::new(10, 280), text_style).draw(target)?;
    eg::image::Image::new(&tile_c, Point::new(50, 250)).draw(target)?;

    Ok(())
}
