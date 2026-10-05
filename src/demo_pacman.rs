//! # Example: Pacman
//!
//! An example displaying an animated Pacman.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_hal as hal;

pub fn run<ColorType, DrawTarget, DelayNs, FnInterval>(target: &mut DrawTarget, mut delay: DelayNs, mut fn_interval: FnInterval)
    -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>, 
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FnInterval: FnMut(&mut DrawTarget) -> bool,
{
    const STEPS: i32 = 10;
    // Create styles used by the drawing operations.
    let style_sector = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::BLACK).stroke_width(2).fill_color(ColorType::YELLOW).build();
    let style_eye = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::BLACK).stroke_width(1).fill_color(ColorType::BLACK).build();
    let mut istep: i32 = 0;
    let bbox = target.bounding_box();
    let center = bbox.center();
    let diameter = core::cmp::min(bbox.size.width, bbox.size.height);
    let offset_eye = Point::new(diameter as i32 * 6 / 100, -(diameter as i32) * 30 / 100);
    let size_eye = diameter * 8 / 100;
    loop {
        target.clear(ColorType::WHITE)?;
        let deg_open = ((30 * (istep - STEPS).abs()) as f32) / STEPS as f32;
        let angle_start = Angle::from_degrees(deg_open);
        let angle_sweep = Angle::from_degrees(360.0 - 2.0 * deg_open);
        eg::primitives::Sector::with_center(center, diameter, angle_start, angle_sweep)
            .into_styled(style_sector).draw(target)?;
        eg::primitives::Circle::with_center(center + offset_eye, size_eye)
            .into_styled(style_eye).draw(target)?;
        istep = (istep + 1) % (2 * STEPS + 1);
        if fn_interval(target) { break; }
        delay.delay_ms(50); // 50 ms delay
    }
    Ok(())
}
