//! # Example: 7-Segment Digital Clock
//!
//! An example displaying a digital clock using the `eg-seven-segment` crate.

use core::fmt::Write as _;
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_hal_async as hal_async;
use heapless::String;

/// Draws a digital clock with the current local time to the specified display
pub async fn run<DrawTarget, ColorType, DelayNs, FnGetTime, FnInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut fn_get_time: FnGetTime, mut fn_interval: FnInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal_async::delay::DelayNs,
    FnGetTime: FnMut() -> (u8, u8, u8),
    FnInterval: FnMut(&mut DrawTarget) -> bool,
{
    let character_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(30, 60)).segment_width(8).segment_color(ColorType::WHITE).build();
    let text_style_top = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).baseline(eg::text::Baseline::Top).build();
    let text_style_bottom = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).baseline(eg::text::Baseline::Bottom).build();
    let mut sbuf = String::<16>::new();
    let mut time_prev = Option::None;
    'running: loop {
        let time_cur = fn_get_time();
        if let Some(time_prev) = time_prev {
            if time_cur == time_prev {
                delay.delay_ms(100).await;
                continue 'running;
            }
        }
        time_prev = Some(time_cur);
        target.clear(ColorType::BLACK)?;
        sbuf.clear();
        eg::text::Text::with_text_style("ABCDEFGHI", target.bounding_box().center() - Point::new(0, 4),
            character_style, text_style_bottom).draw(target)?;
        let (hour, minute, second) = time_cur;
        write!(sbuf, "{:02}:{:02}:{:02}", hour, minute, second).unwrap();
        eg::text::Text::with_text_style(&sbuf, target.bounding_box().center() + Point::new(0, 4),
            character_style, text_style_top).draw(target)?;
        if fn_interval(target) { break 'running; }
        delay.delay_ms(100).await;
    }
    Ok(())
}
