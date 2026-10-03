//! # Example: 7-Segment Digital Clock
//!
//! An example displaying a digital clock using the `eg-seven-segment` crate.

use core::fmt::Write as _;
//use chrono::prelude::*;
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics as eg;
use embedded_graphics::Drawable as _;
use embedded_hal as hal;
use heapless::String;

/// Draws a digital clock with the current local time to the specified display
pub fn run<DrawTarget, ColorType, DelayNs, FuncInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut func_interval: FuncInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FuncInterval: FnMut(&mut DrawTarget) -> bool,
{
/*
    let character_style = SevenSegmentStyleBuilder::new()
        .segment_color(ColorType::WHITE).build();
    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).baseline(eg::text::Baseline::Middle).build();
    let mut sbuf = String::<16>::new();
    loop {
        target.clear(ColorType::BLACK)?;
        let time = Local::now();
        sbuf.clear();
        write!(sbuf, "{:02}:{:02}:{:02}", time.hour(), time.minute(), time.second()).unwrap();
        eg::text::Text::with_text_style(&sbuf, target.bounding_box().center(),
            character_style, text_style).draw(target)?;
        if func_interval(target) { break; }
        delay.delay_ms(100);
    }
*/
    Ok(())
}
