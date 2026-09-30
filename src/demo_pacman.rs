//! # Example: Pacman
//!
//! An example displaying an animated Pacman.
use embedded_graphics as eg;

pub fn run<ColorType, DrawTarget, FuncInterval>(target: &mut DrawTarget, mut func_interval: FuncInterval)
    -> Result<(), DrawTarget::Error>
where
    ColorType: eg::pixelcolor::RgbColor,
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>, 
    FuncInterval: FnMut(&mut DrawTarget) -> bool,
{
    const STEPS: i32 = 10;
    use eg::prelude::*;
    // Create styles used by the drawing operations.
    let style_sector = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::BLACK).stroke_width(2).fill_color(ColorType::YELLOW).build();
    let style_eye = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::BLACK).stroke_width(1).fill_color(ColorType::BLACK).build();
    let mut istep: i32 = 0;
    loop {
        target.clear(ColorType::WHITE)?;
        let deg_open = ((30 * (istep - STEPS).abs()) as f32) / STEPS as f32;
        let angle_start = Angle::from_degrees(deg_open);
        let angle_sweep = Angle::from_degrees(360.0 - 2.0 * deg_open);
        eg::primitives::Sector::new(Point::new(2, 2), 61, angle_start, angle_sweep)
            .into_styled(style_sector).draw(target)?;
        eg::primitives::Circle::new(Point::new(36, 16), 5)
            .into_styled(style_eye).draw(target)?;
        istep = (istep + 1) % (2 * STEPS + 1);
        if func_interval(target) { break; }
    }
    Ok(())
}
