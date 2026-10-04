//! # Example: Hello world
//!
//! A simple hello world example displaying some primitive shapes and some text underneath.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::iso_8859_1 as eg_fonts;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    const SIZE_OBJ: i32 = 64;
    let mut offset = Point::new(10, 14);

    target.clear(ColorType::BLACK)?;

    // Draw a 3px wide outline around the target.
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(3)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build();
    target.bounding_box().into_styled(style).draw(target)?;

    // Draw a triangle.
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(1).build();
    eg::primitives::Triangle::new(Point::new(0, SIZE_OBJ), Point::new(SIZE_OBJ, SIZE_OBJ),
        Point::new(SIZE_OBJ / 2, 0)).translate_mut(offset).into_styled(style).draw(target)?;
    offset.x += SIZE_OBJ + 4;

    // Draw a filled square
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .fill_color(ColorType::WHITE).build();
    eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(SIZE_OBJ as u32, SIZE_OBJ as u32))
        .translate_mut(offset).into_styled(style).draw(target)?;
    offset.x += SIZE_OBJ + 4;

    // Draw a circle with a 3px wide stroke.
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(3).build();
    eg::primitives::Circle::new(Point::new(0, 0), SIZE_OBJ as u32)
        .translate_mut(offset).into_styled(style).draw(target)?;

    // Draw centered text.
    let text = "embedded-graphics";
    let position = target.bounding_box().center() + Point::new(0, 15);
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg_fonts::FONT_10X20).text_color(ColorType::WHITE).build();
    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).build();
    eg::text::Text::with_text_style(text, position, character_style, text_style).draw(target)?;

    Ok(())
}
