//! # Example: Text styles
//!
//! Display a sentence of text using different styles, colors an decorations.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics::mono_font::ascii as eg_fonts;

type ColorType = eg::pixelcolor::Rgb888;

fn draw_text<DrawTarget>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
{
    use eg::prelude::*;
    let font = &eg_fonts::FONT_8X13; 
    let font_bold = &eg_fonts::FONT_8X13_BOLD; 
    let font_italic = &eg_fonts::FONT_8X13_ITALIC; 
    let character_style_normal = eg::mono_font::MonoTextStyleBuilder::new()
        .font(font).text_color(ColorType::WHITE).build();
    // First line
    let mut position_base = Point::new(15, 30 + font.character_size.height as i32);
    let position = position_base;
    let position = {
        eg::text::Text::new("A sentence with normal, ", position, character_style_normal).draw(target)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .text_color(ColorType::CSS_YELLOW).underline().build();
        eg::text::Text::new("yellow underline", position, character_style).draw(target)?
    };
    let position = {
        eg::text::Text::new(", ", position, character_style_normal).draw(target)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .strikethrough_with_color(ColorType::RED).build();
        eg::text::Text::new("red strikethrough", position, character_style).draw(target)?
    };
    let _ = {
        eg::text::Text::new(", ", position, character_style_normal).draw(target)?
    };
    // Second line
    position_base.y += font.character_size.height as i32;
    let position = position_base;
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .font(font_bold).build();
        eg::text::Text::new("bold", position,
            character_style).draw(target)?
    };
    let position = {
        eg::text::Text::new(", ", position, character_style_normal).draw(target)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .text_color(ColorType::CSS_TOMATO).background_color(ColorType::CSS_WHEAT).build();
        eg::text::Text::new("highlighted", position, character_style).draw(target)?
    };
    let position = {
        eg::text::Text::new(" and ", position, character_style_normal).draw(target)?
    };
    let position = {
        let italic = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .font(&font_italic).build();
        eg::text::Text::new("italic", position, italic).draw(target)?
    };
    let _ = {
        eg::text::Text::new(" text!", position, character_style_normal).draw(target)?
    };
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut target = eg_sim::SimulatorDisplay::<ColorType>::new(Size::new(512, 128));
    draw_text(&mut target)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Text styles", &output_settings).show_static(&target);
    Ok(())
}
