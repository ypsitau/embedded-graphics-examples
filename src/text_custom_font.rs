//! # Example: Custom font
//!
//! Shows how to implement a custom `SevenSegmentFont` font using the `MonoFont` struct. This font
//! renders numbers only and emulates a classic 7 segment display.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

const SEVENT_SEGMENT_FONT: eg::mono_font::MonoFont = eg::mono_font::MonoFont {
    image:              eg::image::ImageRaw::new(include_bytes!("assets/seven-segment-font.raw"), 224),
    glyph_mapping:      &eg::mono_font::mapping::StrGlyphMapping::new("0123456789", 0),
    character_size:     Size::new(22, 40),  // (22 * 10 + 7) / 8 * 8 = 224
    character_spacing:  4,
    baseline:           7,
    underline:          eg::mono_font::DecorationDimensions::default_underline(40),
    strikethrough:      eg::mono_font::DecorationDimensions::default_strikethrough(40),
};

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor + eg::pixelcolor::WebColors,
{
    let bbox = target.bounding_box();
    let str_digits = "0123456789";
    let font = &SEVENT_SEGMENT_FONT;

    target.clear(ColorType::BLACK)?;

    let character_style_builder = eg::mono_font::MonoTextStyleBuilder::new()
        .font(font).text_color(ColorType::WHITE).background_color(ColorType::CSS_BROWN);
    let text_style_builder = eg::text::TextStyleBuilder::new().alignment(eg::text::Alignment::Center);

    let character_style = character_style_builder.build();
    let text_style = text_style_builder.baseline(eg::text::Baseline::Top).build();
    eg::text::Text::with_text_style(str_digits, Point::new(bbox.center().x, 0),
        character_style, text_style).draw(target)?;

    let character_style = character_style_builder.strikethrough().build();
    let text_style = text_style_builder.baseline(eg::text::Baseline::Middle).build();
    eg::text::Text::with_text_style(str_digits, bbox.center(),
        character_style, text_style).draw(target)?;

    let character_style = character_style_builder.underline().build();
    let text_style = text_style_builder.baseline(eg::text::Baseline::Bottom).build();
    eg::text::Text::with_text_style(str_digits, Point::new(bbox.center().x, bbox.size.height as i32 - 4),
        character_style, text_style).draw(target)?;

    Ok(())
}
