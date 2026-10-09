//! # Example: Multiline text box using embedded-text
//!
//! This example demonstrates drawing a piece of multiline text that is both horizontally and
//! vertically centered. The example is using the TextBox from embedded-text. For more examples,
//! check out the [embedded-text repository](https://github.com/embedded-graphics/embedded-text).

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_text as embtext;
use eg_seven_segment::SevenSegmentStyleBuilder;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor + eg::pixelcolor::WebColors
        + core::convert::From<eg::pixelcolor::Rgb888>,
{
    let str_text = concat!(
        "0123456789:.\n",
        "ABCDEFGHIJLNOPQRSTUY\n",
        "abcdefghijlnopqrstuy\n",
        "_-=°\"'()[]?",
    );
    let bbox = target.bounding_box();
    target.clear(ColorType::BLACK)?;
    let character_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(14, 24)).digit_spacing(2).segment_width(4)
        .segment_color(ColorType::CSS_WHITE)
        .inactive_segment_color(ColorType::CSS_BLACK)
        .build();
    let textbox_style = embtext::style::TextBoxStyleBuilder::new()
        .alignment(embtext::alignment::HorizontalAlignment::Left)
        .vertical_alignment(embtext::alignment::VerticalAlignment::Top).build();
    embtext::TextBox::with_textbox_style(str_text,
        bbox, character_style, textbox_style).draw(target)?;
    let character_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(30, 60)).digit_spacing(2).segment_width(8)
        .segment_color(ColorType::CSS_RED)
        .inactive_segment_color(eg::pixelcolor::Rgb888::new(64, 0, 0).into())
        .build();
    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Left).baseline(eg::text::Baseline::Bottom).build();
    let pos = Point::new(bbox.top_left.x, bbox.top_left.y + bbox.size.height as i32);
    let mut pos = eg::text::Text::with_text_style("12:34", pos, character_style, text_style).draw(target)?;
    pos.x += 10;
    let character_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(26, 60)).digit_spacing(4).segment_width(8)
        .segment_color(ColorType::CSS_LIGHT_GREEN)
        .inactive_segment_color(eg::pixelcolor::Rgb888::new(0, 64, 0).into())
        .build();
    let _pos = eg::text::Text::with_text_style("24.5°C", pos, character_style, text_style).draw(target)?;
    Ok(())
}
