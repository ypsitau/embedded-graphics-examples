//! # Example: Fonts
//!
//! Demonstrate some of the available builtin fonts. A full list of fonts can be found in the
//! [embedded-graphics documentation](https://docs.rs/embedded-graphics).

use embedded_graphics as eg;
use embedded_graphics::mono_font::ascii as eg_fonts;
use embedded_graphics_simulator as eg_sim;

type ColorType = eg::pixelcolor::BinaryColor;

fn draw_text<DrawTarget>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
{
    const LINE_SPACING: i32 = 4;
    use eg::prelude::*;
    let entries = [
        ("Hello World! - FONT_4X6", &eg_fonts::FONT_4X6),
        ("Hello World! - FONT_5X8", &eg_fonts::FONT_5X8),
        ("Hello World! - FONT_6X12", &eg_fonts::FONT_6X12),
        ("Hello World! - FONT_7X13", &eg_fonts::FONT_7X13),
        ("Hello World! - FONT_8X13", &eg_fonts::FONT_8X13),
        ("Hello World! - FONT_9X15", &eg_fonts::FONT_9X15),
        ("Hello World! - FONT_10X20", &eg_fonts::FONT_10X20),
    ];
    let mut pt = Point::new(15, 4);
    for (text, font) in entries.iter() {
        pt.y += font.character_size.height as i32;
        let character_style = eg::mono_font::MonoTextStyleBuilder::new()
            .font(font).text_color(ColorType::On).build();
        eg::text::Text::new(text, pt, character_style).draw(target)?;
        pt.y += font.character_size.height as i32 + LINE_SPACING;
        let character_style = eg::mono_font::MonoTextStyleBuilder::new()
            .font(font).text_color(ColorType::Off).background_color(ColorType::On).build();
        eg::text::Text::new(text, pt, character_style).draw(target)?;
        pt.y += LINE_SPACING;
    }
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut target = eg_sim::SimulatorDisplay::<ColorType>::new(Size::new(350, 240));
    draw_text(&mut target)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Fonts", &output_settings).show_static(&target);
    Ok(())
}
