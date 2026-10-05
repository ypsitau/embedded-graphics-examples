//! # Example: 7-Segment Digital Clock
//!
//! An example displaying a digital clock using the `eg-seven-segment` crate.

use core::fmt::Write as _;
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_hal as hal;
use heapless::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    /// Creates a new `Time` instance.
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self { hour, minute, second }
    }
}

/// Draws a digital clock with the current local time to the specified display
pub fn run<DrawTarget, ColorType, DelayNs, FnGetTime, FnInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut fn_get_time: FnGetTime, mut fn_interval: FnInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FnGetTime: FnMut() -> Time,
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
                delay.delay_ms(100);
                continue 'running;
            }
        }
        time_prev = Some(time_cur);
        target.clear(ColorType::BLACK)?;
        sbuf.clear();
        eg::text::Text::with_text_style("ABCDEFGHI", target.bounding_box().center() - Point::new(0, 4),
            character_style, text_style_bottom).draw(target)?;
        write!(sbuf, "{:02}:{:02}:{:02}", time_cur.hour, time_cur.minute, time_cur.second).unwrap();
        eg::text::Text::with_text_style(&sbuf, target.bounding_box().center() + Point::new(0, 4),
            character_style, text_style_top).draw(target)?;
        if fn_interval(target) { break 'running; }
        delay.delay_ms(100);
    }
    Ok(())
}
