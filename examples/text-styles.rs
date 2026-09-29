//! # Example: Text styles
//!
//! Display a sentence of text using different styles, colors an decorations.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

type ColorType = eg::pixelcolor::Rgb888;

fn draw_text<Display>(display: &mut Display) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = ColorType>,
{
    use eg::prelude::*;
    let font = &eg::mono_font::ascii::FONT_8X13; 
    let font_bold = &eg::mono_font::ascii::FONT_8X13_BOLD; 
    let font_italic = &eg::mono_font::ascii::FONT_8X13_ITALIC; 
    let character_style_normal = eg::mono_font::MonoTextStyleBuilder::new()
        .font(font).text_color(ColorType::WHITE).build();
    // First line
    let mut position_base = Point::new(15, 30 + font.character_size.height as i32);
    let position = position_base;
    let position = {
        eg::text::Text::new("A sentence with normal, ", position, character_style_normal).draw(display)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .text_color(ColorType::CSS_YELLOW).underline().build();
        eg::text::Text::new("yellow underline", position, character_style).draw(display)?
    };
    let position = {
        eg::text::Text::new(", ", position, character_style_normal).draw(display)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .strikethrough_with_color(ColorType::RED).build();
        eg::text::Text::new("red strikethrough", position, character_style).draw(display)?
    };
    let _ = {
        eg::text::Text::new(", ", position, character_style_normal).draw(display)?
    };
    // Second line
    position_base.y += font.character_size.height as i32;
    let position = position_base;
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .font(font_bold).build();
        eg::text::Text::new("bold", position,
            character_style).draw(display)?
    };
    let position = {
        eg::text::Text::new(", ", position, character_style_normal).draw(display)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .text_color(ColorType::CSS_TOMATO).background_color(ColorType::CSS_WHEAT).build();
        eg::text::Text::new("highlighted", position, character_style).draw(display)?
    };
    let position = {
        eg::text::Text::new(" and ", position, character_style_normal).draw(display)?
    };
    let position = {
        let italic = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .font(&font_italic).build();
        eg::text::Text::new("italic", position, italic).draw(display)?
    };
    let _ = {
        eg::text::Text::new(" text!", position, character_style_normal).draw(display)?
    };
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::<ColorType>::new(Size::new(512, 128));
    draw_text(&mut display)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Text styles", &output_settings).show_static(&display);
    Ok(())
}
