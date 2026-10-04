//! # Example: Text alignment
//!
//! Draw left/center/right aligned text in a containing box.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let bounding_box = target.bounding_box();

    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg::mono_font::ascii::FONT_10X20)
        .text_color(ColorType::GREEN).build();

    target.clear(ColorType::BLACK)?;

    let text_style = eg::text::TextStyleBuilder::new()
        .baseline(eg::text::Baseline::Top).alignment(eg::text::Alignment::Left).build();
    eg::text::Text::with_text_style("Top+Left",
        bounding_box.top_left, character_style, text_style).draw(target)?;

    let text_style = eg::text::TextStyleBuilder::new()
        .baseline(eg::text::Baseline::Middle).alignment(eg::text::Alignment::Center).build();
    eg::text::Text::with_text_style("Middle+Center",
        bounding_box.center(), character_style, text_style).draw(target)?;

    let text_style = eg::text::TextStyleBuilder::new()
        .baseline(eg::text::Baseline::Bottom).alignment(eg::text::Alignment::Right).build();
    eg::text::Text::with_text_style("Bottom+Right",
        bounding_box.bottom_right().unwrap(), character_style, text_style).draw(target)?;

    Ok(())
}
