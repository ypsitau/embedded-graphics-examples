//! # Example: Analog Clock
//!
//! ![Screenshot of clock example]( data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfwAAAH9AQMAAADFwFz1AAAABlBMVEUAAAD///+l2Z/dAAAFM0lEQVR42u3dYWrjMBAF4Afz1zBX8QEEc/WBvdAcQKBNussK3EbVpDNSt+SRYMWWPrBsybQJCK+88sp/EWm32BagN+/EcoBuLRUE8K2wA6B7c9yBN2I9QK3+2eItrS0HWv0L9c9LgV6f+p6lADdcAWq6AuiVO9DRhYBYty571wBc8RGApquApkM4H+A6lvOBpkM6H+A6tvOBpkM8H6A61BcAosMetnSA6olBzpYOsI0B0Wyg4cAgSjUZIPsEgCQDoqCRD7DlAvX2Hlyo414lFSD7HJAsoI/lc3An9xGdBFRMpCYCZDOAJAKsMwBrHiCYCVka0I+MI0GAvw97vSxAMBeyLKBiMjUJIJsFJAjwX8VeMwcQzIYsB6iYTk0ByOYBCQL8V7HXzQAE8yHLAAocSQHMA5QEgNQDcALAPkDjAYEnZPFAhSs1HjAfUMIBUh/A4QB7AY0GBL6QRQMFzoQD5gVKMEBuQKIB9QIcDDC8IY0FxA9YLFDgTjBQ/UCNBcwPlFCA1A9wKMBPAE1DgeI/AwoFxN+LJ1kkUHD655NQoLrntAOokYAB6h9NgQAZnohEAvoMwFHA+9HcHuUCaCCAZwAKBOQ5wOKAcgGU7q/WLoULgB8FXMZSlVqkisq1cAHKDwJIrzdSlXZvdysU6YUrwD8ZqHR/3VKpKeRvgVoawHgOIE0DfrX7S6X35q/bpkoaIM8CFgWUy4G3VnegSr21fSt8BOAnA2+75O9cet9WQG7lNMCeBUoW0B7kBwOkzwIcB/iSD+h2oKwGnv8XMWkSgHM7wNsB2Fqg4OlzsDQAuh0o2wHS3QDO7QCvBGxw7JOUTODYDpDuBlC2A6S7AZzbAd4OwBYB/XL5xxMnA6S7AZTtAG0HYNsB3g7AWDcDR6ubAfoGQFsAtGbPAdLal4Fv0gcjAK1+EfgGN9IXgcK6GVBsBhi7gXM7oDdjK8DYDZxrANioD8dASQYYO4F/bY+NAOlu4MBu4NwNkG4FekvKBUZ/WtkEYJkAYTdwbAesz6x7AMJu4FgJ8INDnwOkeQBhN1B68dwD6FKAdNCHQ4CRBpTtgO4G+DKws4H3T9dzEihpgO4GGKuBMujDIWBZgF6G9nKAsRyQQR+OAAoD+DJXTwOaAxxYCPTCaBLVwYyWAZDuBg5sAGDosXmgpACELUAv4XB8fWwpgO0B5NKHcwAFAtz70ANoIPCvaA6AA4E+O3q+QWfEAf2CeAAJBPAUUCKBCqgPOIAaCRQwfAApLBIQnHD+iOCkUIBJvQCThgIVXgAtFCD1A4xIYDyaPtZLLFD9QB0CbqD4AYsFxA1QMMB+QGOB/nH26coIBswLSDAAN1CigeIFLBoQJ0DhAKvv6coaDZAXQDQA8wElHqg+oMYD4gLI4gF2/daTNR4gH4B4AOYBSgZQPIBlAOIAKALw3En6vm4GQDYPCDIA1Hmg5gAyDZDlAKyzAGsOQDb7fBfkAKizQM0CZBIgywJY5wDWLIBsDhBkAZCp5ztZHsD6ALZxLT/g7UU9LicaALiGE4EulfIAUQD6wVXUfj3ZMgGy2xuPAep9mASgfg7UXEAUxwCAgi0XIMM5Ak4IcgE0nKNb+aSaDbDpaDgfotkAVR1NaUdDBOBfWkj7GeYDVEdAQz6Apo+frlxTgUGls+OpwONaZ7dXAFzfA51eAfR6HehyCOBfdu/o8BIAYleg710DUNP3ADesAnrlDnR0EYBWL0Dfkw90oQPU24cA/uVYqTV8Ffj/1rT9Buv6foO1jb/D+s6vvPKKI78B89G0YRkxkl8AAAAASUVORK5CYII=)
//!
//! This example shows some more advanced usage of Embedded Graphics. It draws a round clock face
//! with hour, minute and second hands. A digital clock is drawn in the middle of the clock. The
//! whole thing is updated with your computer's local time every 50ms.

use core::fmt::Write as _;
use heapless::String;
use core::f32::consts::PI;
use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_hal as hal;
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
use micromath::F32Ext;

use embedded_graphics::mono_font::ascii as fonts;

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

pub fn run<DrawTarget, ColorType, DelayNs, FnGetTime, FnInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut fn_get_time: FnGetTime, mut fn_interval: FnInterval)
-> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FnGetTime: FnMut() -> Time,
    FnInterval: FnMut(&mut DrawTarget) -> bool,
{
    /// The margin between the clock face and the display border.
    const MARGIN: u32 = 10;
    // The draw target bounding box can be used to determine the size of the display.
    let bounding_box = target.bounding_box();
    let diameter = bounding_box.size.width.min(bounding_box.size.height) - 2 * MARGIN;
    let circle_clock_face = eg::primitives::Circle::with_center(bounding_box.center(), diameter);
    let style_erase = eg::primitives::PrimitiveStyle::with_fill(ColorType::BLACK);
    let mut time_prev = Option::None;
    target.clear(ColorType::BLACK)?;
    'running: loop {
        let time_cur = fn_get_time();
        if let Some(time_prev) = time_prev {
            if time_cur == time_prev {
                delay.delay_ms(100);
                continue 'running;
            }
        }
        time_prev = Some(time_cur);
        // Calculate the position of the three clock hands in radians.
        let hour_radian = hour_to_radian(time_cur.hour as u32);
        let minute_radian = sexagesimal_to_radian(time_cur.minute as u32);
        let second_radian = sexagesimal_to_radian(time_cur.second as u32);
        let mut str_time: String<16> = String::new();
        write!(str_time, "{:02}:{:02}:{:02}", time_cur.hour, time_cur.minute, time_cur.second).unwrap();
        //target.clear(ColorType::BLACK)?;
        circle_clock_face.bounding_box().into_styled(style_erase).draw(target)?;
        draw_face(target, &circle_clock_face)?;
        draw_hand(target, &circle_clock_face, hour_radian, -60)?;
        draw_hand(target, &circle_clock_face, minute_radian, -30)?;
        draw_hand(target, &circle_clock_face, second_radian, 0)?;
        draw_second_decoration(target, &circle_clock_face, second_radian, -20)?;
        // Draw digital clock just above center.
        draw_digital_clock(target, &circle_clock_face, &str_time)?;
        // Draw a small circle over the hands in the center of the clock face.
        // This has to happen after the hands are drawn so they're covered up.
        eg::primitives::Circle::with_center(circle_clock_face.center(), 9)
            .into_styled(eg::primitives::PrimitiveStyle::with_fill(ColorType::WHITE))
            .draw(target)?;
        if fn_interval(target) { break 'running Ok(()); }
        delay.delay_ms(100);
    }
}

/// Converts a polar coordinate (angle/distance) into an (X, Y) coordinate centered around the
/// center of the circle.
///
/// The angle is relative to the 12 o'clock position and the radius is relative to the edge of the
/// clock face.
fn polar(circle: &eg::primitives::Circle, angle: f32, radius_delta: i32) -> Point {
    let radius = circle.diameter as f32 / 2.0 + radius_delta as f32;
    circle.center() + Point::new((angle.sin() * radius) as i32, -(angle.cos() * radius) as i32)
}

/// Converts an hour into an angle in radians.
fn hour_to_radian(hour: u32) -> f32 {
    ((hour % 12) as f32 / 12.0) * 2.0 * PI
}

/// Converts a sexagesimal (base 60) value into an angle in radians.
fn sexagesimal_to_radian(value: u32) -> f32 {
    (value as f32 / 60.0) * 2.0 * PI
}

/// Draws a circle and 12 graduations as a simple clock face.
fn draw_face<DrawTarget, ColorType>(target: &mut DrawTarget, circle_clock_face: &eg::primitives::Circle) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    // Draw the outer face.
    let style_builder = eg::primitives::PrimitiveStyleBuilder::new().stroke_color(ColorType::WHITE);
    circle_clock_face.into_styled(style_builder.stroke_width(2).build()).draw(target)?;
    // Draw 12 graduations.
    for angle in (0..12).map(hour_to_radian) {
        // Start point on circumference.
        let start = polar(circle_clock_face, angle, 0);
        // End point offset by 10 pixels from the edge.
        let end = polar(circle_clock_face, angle, -10);
        eg::primitives::Line::new(start, end).into_styled(style_builder.stroke_width(1).build()).draw(target)?;
    }
    Ok(())
}

/// Draws a clock hand.
fn draw_hand<DrawTarget, ColorType>(target: &mut DrawTarget, circle_clock_face: &eg::primitives::Circle, angle: f32, length_delta: i32)
    -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let start = circle_clock_face.center();
    let end = polar(circle_clock_face, angle, length_delta);
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(1).build();
    eg::primitives::Line::new(start, end).into_styled(style).draw(target)
}

/// Draws a decorative circle on the second hand.
fn draw_second_decoration<DrawTarget, ColorType>(target: &mut DrawTarget, circle_clock_face: &eg::primitives::Circle, angle: f32, length_delta: i32)
    -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let center = polar(circle_clock_face, angle, length_delta);
    let diameter = 11;
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(1).fill_color(ColorType::BLACK).build();
    // Draw a fancy circle near the end of the second hand.
    eg::primitives::Circle::with_center(center, diameter).into_styled(style).draw(target)
}

/// Draw digital clock just above center with black text on a white background
fn draw_digital_clock<DrawTarget, ColorType>(target: &mut DrawTarget, circle_clock_face: &eg::primitives::Circle, str_time: &str)
    -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    // Create a styled text object for the time text.
    let position = circle_clock_face.center() - circle_clock_face.bounding_box().size.y_axis() / 4;
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&fonts::FONT_9X15).text_color(ColorType::BLACK).build();
    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).baseline(eg::text::Baseline::Middle).build();
    let text = eg::text::Text::with_text_style(&str_time, position, character_style, text_style);
    // Add a background around the time digits.
    // Note that there is no bottom-right padding as this is added by the font renderer itself.
    let bbox_text = text.bounding_box();
    let style = eg::primitives::PrimitiveStyleBuilder::new().fill_color(ColorType::WHITE).build();
    eg::primitives::Rectangle::with_center(bbox_text.center(), bbox_text.size + Size::new(4, 4))
        .into_styled(style).draw(target)?;
    // Draw the text after the background is drawn.
    text.draw(target)?;
    Ok(())
}
