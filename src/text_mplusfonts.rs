//! # Example: Using M+ Fonts with Embedded-Graphics and Embedded-Text
//!
//! This example demonstrates how to use the M+ fonts with embedded-graphics and embedded-text.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_text as embtext;

#[mplusfonts::strings]
pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor + Default
        + mplusfonts::color::Invert + mplusfonts::color::Screen + mplusfonts::color::WeightedAvg,
    mplusfonts::color::Colormap<ColorType, 16>: mplusfonts::color::Linear<ColorType>,
{
    let str_text = indoc::indoc! {"
    親譲りの無鉄砲で小供の時から損ばかりしている。小学校に居る時分学校の二階から飛び降りて一週間ほど腰を抜かした事がある。\
    なぜそんな無闇をしたと聞く人があるかも知れぬ。別段深い理由でもない。\
    "};
    target.clear(ColorType::BLACK)?;
    #[strings::emit]
    let font_thin = mplusfonts::mplus!(code(100), THIN, code_line_height(16), true, 1, 4);
    #[strings::emit]
    let font_light = mplusfonts::mplus!(code(100), LIGHT, code_line_height(16), true, 1, 4);
    #[strings::emit]
    let font_medium = mplusfonts::mplus!(code(100), MEDIUM, code_line_height(16), true, 1, 4);
    #[strings::emit]
    let font_bold = mplusfonts::mplus!(code(100), BOLD, code_line_height(16), true, 1, 4);
    let textbox_style = embtext::style::TextBoxStyleBuilder::new()
        .alignment(embtext::alignment::HorizontalAlignment::Left)
        .vertical_alignment(embtext::alignment::VerticalAlignment::Top).build();
    let mut y_offset = 0;
    for font in [font_thin, font_light, font_medium, font_bold].iter() {
        let character_style = mplusfonts::style::BitmapFontStyleBuilder::new()
            .font(font).text_color(ColorType::WHITE).build();
        embtext::TextBox::with_textbox_style(str_text,
            target.bounding_box().translate(Point::new(0, y_offset)), character_style, textbox_style).draw(target)?;
        y_offset += 60;
    }
    Ok(())
}
