//! # Example: Polyline
//!
//! This example draws a star shape using the `Polyline` primitive

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use core::f32::consts;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor
        + From<eg::pixelcolor::Rgb555> + From<eg::pixelcolor::Rgb565> + From<eg::pixelcolor::Rgb888>,
{
/*
    let bbox = target.bounding_box();
    let size = core::cmp::min(bbox.size.width, bbox.size.height) as i32;
    let line_style = eg::primitives::PrimitiveStyle::with_stroke(ColorType::GREEN, 7);
    let center = bbox.center();
    let points = {
        let radius_outer = size as f32 / 3.0;
        let radius_inner = radius_outer * 0.38;
        let mut points = [Point::zero(); 11];
        for (index, point) in points.iter_mut().enumerate() {
            let radian = index as f32 * consts::PI / 5.0 - consts::PI / 2.0;
            let radius = if index % 2 == 0 { radius_outer } else { radius_inner };
            let x = (radius * radian.cos()).round() as i32;
            let y = (radius * radian.sin()).round() as i32;
            *point = Point::new(x, y);
        }
        points
    };
    eg::primitives::Polyline::new(&points).translate(center).into_styled(line_style).draw(target)?;
*/
    Ok(())
}
