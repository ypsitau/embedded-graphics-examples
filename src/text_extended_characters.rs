//! # Example: Extended characters
//!
//! Demonstrate ability of all built in fonts to render extended characters.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::iso_8859_1 as eg_fonts;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let test_text  = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
    let tbl = [
        ("Font6x9", &eg_fonts::FONT_6X9),
        ("Font6x13", &eg_fonts::FONT_6X13),
        ("Font8x13", &eg_fonts::FONT_8X13),
        ("Font10x20", &eg_fonts::FONT_10X20),
    ];
    let mut position = Point::new(0, 15);
    for (name, font) in &tbl {
        position.y += font.character_size.height as i32;
        let text_style = eg::mono_font::MonoTextStyle::new(font, ColorType::WHITE);
        eg::text::Text::new(name, position, text_style).draw(target)?;
        position.y += font.character_size.height as i32 + 4;
        eg::text::Text::new(test_text, position, text_style).draw(target)?;
        position.y += 10;
    }
    Ok(())
}
