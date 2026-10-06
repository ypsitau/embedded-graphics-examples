//! # Example: Fonts
//!
//! Demonstrate some of the available builtin fonts. A full list of fonts can be found in the
//! [embedded-graphics documentation](https://docs.rs/embedded-graphics).

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    use u8g2_fonts::fonts;
    use u8g2_fonts::types;
    use u8g2_fonts::FontRenderer as FR;
    const LINE_SPACING: i32 = 2;
    let entries = [
        ("Hello World! - micro_tr",     FR::new::<fonts::u8g2_font_micro_tr>()),
        ("Hello World! - 4x6_tr",       FR::new::<fonts::u8g2_font_4x6_tr>()),
        ("Hello World! - 6x10_tr",      FR::new::<fonts::u8g2_font_6x10_tr>()),
        ("Hello World! - profont12_tr", FR::new::<fonts::u8g2_font_profont12_tr>()),
        ("Hello World! - helvR14_tf",   FR::new::<fonts::u8g2_font_helvR14_tf>()),
        ("Hello World! - helvB14_tf",   FR::new::<fonts::u8g2_font_helvB14_tf>()),
        ("Hello World! - ncenR14_tr",   FR::new::<fonts::u8g2_font_ncenR14_tr>()),
        ("Hello World! - ncenB14_tr",   FR::new::<fonts::u8g2_font_ncenB14_tr>()),
        ("今日は, 世界! - unifont_t_japanese1",   FR::new::<fonts::u8g2_font_unifont_t_japanese1>()),
        ("@ABCDEFGHIJKLMNOPQRSTUVWXYZ", FR::new::<fonts::u8g2_font_open_iconic_arrow_1x_t>()),
        ("@ABCDEFGHIJKLMNOPQRSTUVWXYZ", FR::new::<fonts::u8g2_font_open_iconic_gui_1x_t>()),
    ];
    let mut pt = Point::new(15, 4);
    target.clear(ColorType::BLACK)?;
    for (text, font_renderer) in entries.iter() {
        let text = *text;
        font_renderer.render_aligned(text, pt,
            types::VerticalPosition::Top, types::HorizontalAlignment::Left,
            types::FontColor::Transparent(ColorType::WHITE), target,
        ).unwrap_or_else(|_| panic!("Font rendering failed"));
        pt.y += font_renderer.get_default_line_height() as i32 + LINE_SPACING;
    }
    Ok(())
}
