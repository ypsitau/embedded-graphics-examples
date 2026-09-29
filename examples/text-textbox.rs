//! # Example: Multiline text box using embedded-text
//!
//! This example demonstrates drawing a piece of multiline text that is both horizontally and
//! vertically centered. The example is using the TextBox from embedded-text. For more examples,
//! check out the [embedded-text repository](https://github.com/embedded-graphics/embedded-text).

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;
use embedded_text as embtext;

type ColorType = eg::pixelcolor::BinaryColor;

fn draw_text<DrawTarget>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
{
    use eg::prelude::*;
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg::mono_font::ascii::FONT_6X10).text_color(ColorType::On).build();
    let textbox_style = embtext::style::TextBoxStyleBuilder::new()
        .alignment(embtext::alignment::HorizontalAlignment::Center)
        .vertical_alignment(embtext::alignment::VerticalAlignment::Middle).build();
    embtext::TextBox::with_textbox_style("This is a\nmultiline\nHello World!",
        target.bounding_box(), character_style, textbox_style).draw(target)?;
    Ok(())
}


fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::new(Size::new(128, 128));
    draw_text(&mut display)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("TextBox alignment", &output_settings).show_static(&display);
    Ok(())
}
