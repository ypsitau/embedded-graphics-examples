//! # Example: Transparent fonts
//!
//! Demonstrate the background styles and transparency behaviors of different font styles.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii as eg_fonts;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let style = eg::primitives::PrimitiveStyleBuilder::new().fill_color(ColorType::RED).build();
    eg::primitives::Circle::new(Point::new(40, 80), 80).into_styled(style).draw(target)?;
    eg::primitives::Rectangle::new(Point::new(140, 80), Size::new(80, 80))
        .into_styled(style).draw(target)?;
    let font = &eg_fonts::FONT_8X13;
    let character_style_builder = eg::mono_font::MonoTextStyleBuilder::new().font(font);

    let mut y = 100;
    let character_style = character_style_builder
        .text_color(ColorType::WHITE).build();
    eg::text::Text::new("Hello world! - no background",
        Point::new(15, y), character_style).draw(target)?;
    y += font.character_size.height as i32 + 5;

    let character_style = character_style_builder
        .text_color(ColorType::YELLOW).background_color(ColorType::BLUE).build();
    eg::text::Text::new("Hello world! - filled background 1",
        Point::new(15, y), character_style).draw(target)?;
    y += font.character_size.height as i32 + 5;

    let character_style = character_style_builder
        .text_color(ColorType::BLUE).background_color(ColorType::YELLOW).build();
    eg::text::Text::new("Hello world! - filled background 2",
        Point::new(15, y), character_style).draw(target)?;
    Ok(())
}
