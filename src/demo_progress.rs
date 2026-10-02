//! # Example: Progress
//!
//! An example displaying a progress circle.

use core::fmt::Write as _;
use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii as eg_fonts;
use embedded_hal as hal;
use heapless::String;

pub fn run<DrawTarget, ColorType, DelayNs, FuncInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut func_interval: FuncInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FuncInterval: FnMut(&mut DrawTarget) -> bool,
{
    let center = target.bounding_box().center();
    let style_arc = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(5)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build();
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg_fonts::FONT_10X20).text_color(ColorType::WHITE).build();
    let text_style = eg::text::TextStyleBuilder::new()
        .baseline(eg::text::Baseline::Middle).alignment(eg::text::Alignment::Center).build();
    let mut progress = 0;
    let mut sbuf = String::<32>::new();
    'running: loop {
        target.clear(ColorType::BLACK)?;
        let angle_sweep = progress as f32 * 360.0 / 100.0;
        eg::primitives::Arc::with_center(center, 80, 90.0.deg(), angle_sweep.deg())
            .into_styled(style_arc).draw(target)?;
        sbuf.clear();
        write!(sbuf, "{}%", progress).unwrap();
        eg::text::Text::with_text_style(&sbuf, center, character_style, text_style).draw(target)?;
        if func_interval(target) { break 'running; }
        delay.delay_ms(50);
        progress = (progress + 1) % 101;
    }
    Ok(())
}
