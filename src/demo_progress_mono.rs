//! # Example: Progress
//!
//! An example displaying a progress circle.

use core::fmt::Write as _;
use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii as eg_fonts;
use embedded_hal_async as hal_async;
use heapless::String;

type ColorType = eg::pixelcolor::BinaryColor;

pub async fn run<DrawTarget, DelayNs, FnInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut fn_interval: FnInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    DelayNs: hal_async::delay::DelayNs,
    FnInterval: FnMut(&mut DrawTarget) -> bool,
{
    let center = target.bounding_box().center();
    let style_arc = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::On).stroke_width(5)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build();
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg_fonts::FONT_10X20).text_color(ColorType::On).build();
    let text_style = eg::text::TextStyleBuilder::new()
        .baseline(eg::text::Baseline::Middle).alignment(eg::text::Alignment::Center).build();
    let mut progress = 0;
    let mut sbuf = String::<32>::new();
    'running: loop {
        target.clear(ColorType::Off)?;
        let angle_sweep = progress as f32 * 360.0 / 100.0;
        eg::primitives::Arc::with_center(center, 60, 90.0.deg(), angle_sweep.deg())
            .into_styled(style_arc).draw(target)?;
        sbuf.clear();
        write!(sbuf, "{}%", progress).unwrap();
        eg::text::Text::with_text_style(&sbuf, center, character_style, text_style).draw(target)?;
        if fn_interval(target) { break 'running; }
        delay.delay_ms(50).await;
        progress = (progress + 1) % 101;
    }
    Ok(())
}
