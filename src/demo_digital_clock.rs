//! # Example: 7-Segment Digital Clock
//!
//! An example displaying a digital clock using the `eg-seven-segment` crate.

use core::fmt::Write as _;
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics as eg;
use embedded_graphics::Drawable as _;
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
pub fn run<DrawTarget, ColorType, DelayNs, FuncGetTime, FuncInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut func_get_time: FuncGetTime, mut func_interval: FuncInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FuncGetTime: FnMut() -> Time,
    FuncInterval: FnMut(&mut DrawTarget) -> bool,
{
    let character_style = SevenSegmentStyleBuilder::new()
        .segment_color(ColorType::WHITE).build();
    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).baseline(eg::text::Baseline::Middle).build();
    let mut sbuf = String::<16>::new();
    let mut time_prev = Option::None;
    'running: loop {
        let time_cur = func_get_time();
        if let Some(time_prev) = time_prev {
            if time_cur == time_prev {
                delay.delay_ms(100);
                continue 'running;
            }
        }
        time_prev = Some(time_cur);
        target.clear(ColorType::BLACK)?;
        sbuf.clear();
        write!(sbuf, "{:02}:{:02}:{:02}", time_cur.hour, time_cur.minute, time_cur.second).unwrap();
        eg::text::Text::with_text_style(&sbuf, target.bounding_box().center(),
            character_style, text_style).draw(target)?;
        if func_interval(target) { break 'running; }
        delay.delay_ms(100);
    }
    Ok(())
}
