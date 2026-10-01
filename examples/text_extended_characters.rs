//! # Example: Extended characters
//!
//! Demonstrate ability of all built in fonts to render extended characters.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics::mono_font::iso_8859_1 as eg_fonts;

type ColorType = eg::pixelcolor::BinaryColor;

fn draw_text<DrawTarget>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
{
    use eg::prelude::*;
    let test_text  = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
    let tbl = [
        ("Font6x9", &eg_fonts::FONT_6X9),
        ("Font6x13", &eg_fonts::FONT_6X13),
        ("Font8x13", &eg_fonts::FONT_8X13),
        ("Font10x20", &eg_fonts::FONT_10X20),
    ];
    let mut position = Point::new(15, 15);
    for (name, font) in &tbl {
        position.y += font.character_size.height as i32;
        eg::text::Text::new(&format!("{} {}", name, test_text), position,
            eg::mono_font::MonoTextStyle::new(font, ColorType::On),
        ).draw(target)?;
        position.y += 10; // Add some spacing between different font renderings
    }
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::<ColorType>::new(Size::new(1400, 160));
    draw_text(&mut display)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(1).build();
    eg_sim::Window::new("Fonts", &output_settings).show_static(&display);
    Ok(())
}
