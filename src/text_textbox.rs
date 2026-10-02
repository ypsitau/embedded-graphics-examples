//! # Example: Multiline text box using embedded-text
//!
//! This example demonstrates drawing a piece of multiline text that is both horizontally and
//! vertically centered. The example is using the TextBox from embedded-text. For more examples,
//! check out the [embedded-text repository](https://github.com/embedded-graphics/embedded-text).

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii as eg_fonts;
use embedded_text as embtext;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let text =
        "This example demonstrates drawing a piece of multiline text \
        using the TextBox from embedded-text. For more examples, \
        check out the embedded-text repository.";
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg_fonts::FONT_6X10).text_color(ColorType::WHITE).build();
    let textbox_style = embtext::style::TextBoxStyleBuilder::new()
        .alignment(embtext::alignment::HorizontalAlignment::Left)
        .vertical_alignment(embtext::alignment::VerticalAlignment::Top).build();
    embtext::TextBox::with_textbox_style(text,
        target.bounding_box(), character_style, textbox_style).draw(target)?;
    let textbox_style = embtext::style::TextBoxStyleBuilder::new()
        .alignment(embtext::alignment::HorizontalAlignment::Center)
        .vertical_alignment(embtext::alignment::VerticalAlignment::Middle).build();
    embtext::TextBox::with_textbox_style(text,
        target.bounding_box(), character_style, textbox_style).draw(target)?;
    let textbox_style = embtext::style::TextBoxStyleBuilder::new()
        .alignment(embtext::alignment::HorizontalAlignment::Right)
        .vertical_alignment(embtext::alignment::VerticalAlignment::Bottom).build();
    embtext::TextBox::with_textbox_style(text,
        target.bounding_box(), character_style, textbox_style).draw(target)?;
    Ok(())
}
