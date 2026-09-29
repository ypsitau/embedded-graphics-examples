//! # Example: Pacman
//!
//! An example displaying an animated Pacman.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

// the number of steps of the animation
const STEPS: i32 = 10;

fn draw_pacman<DrawTarget, FuncFlush>(target: &mut DrawTarget, mut func_flush: FuncFlush)
    -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb565>,
    FuncFlush: FnMut(&mut DrawTarget) -> bool,
{
    use eg::prelude::*;
    // Create styles used by the drawing operations.
    let style_sector = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb565::BLACK).stroke_width(2).fill_color(eg::pixelcolor::Rgb565::YELLOW).build();
    let style_eye = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb565::BLACK).stroke_width(1).fill_color(eg::pixelcolor::Rgb565::BLACK).build();
    let mut progress: i32 = 0;
    loop {
        target.clear(eg::pixelcolor::Rgb565::WHITE)?;
        let p = (progress - STEPS).abs();
        let angle_start = Angle::from_degrees((p * 30 / STEPS) as f32);
        let angle_end = Angle::from_degrees((360 - 2 * p * 30 / STEPS) as f32);
        eg::primitives::Sector::new(Point::new(2, 2), 61, angle_start, angle_end)
            .into_styled(style_sector)
            .draw(target)?;
        eg::primitives::Circle::new(Point::new(36, 16), 5).into_styled(style_eye).draw(target)?;
        if func_flush(target) { break; }
        std::thread::sleep(std::time::Duration::from_millis(50));
        progress = (progress + 1) % (2 * STEPS + 1);
    }
    Ok(())
}


fn main() -> Result<(), std::convert::Infallible> {
    use eg::prelude::*;
    // Create a new simulator display with 65x65 pixels.
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(4).build();
    let mut window = eg_sim::Window::new("Pacman", &output_settings);
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb565>::new(Size::new(65, 65));
    draw_pacman(&mut target, |target| {
        window.update(target);
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    })?;
    Ok(())
}
