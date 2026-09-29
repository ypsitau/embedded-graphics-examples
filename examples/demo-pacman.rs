//! # Example: Pacman
//!
//! An example displaying an animated Pacman.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

type ColorType = eg::pixelcolor::Rgb565;

fn draw_pacman<DrawTarget, FuncInterval>(target: &mut DrawTarget, mut func_interval: FuncInterval)
    -> Result<(), DrawTarget::Error>
where
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


fn main() -> Result<(), std::convert::Infallible> {
    use eg::prelude::*;
    // Create a new simulator display with 65x65 pixels.
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(4).build();
    let mut window = eg_sim::Window::new("Pacman", &output_settings);
    let mut target = eg_sim::SimulatorDisplay::<ColorType>::new(Size::new(65, 65));
    draw_pacman(&mut target, |target| {
        window.update(target);
        std::thread::sleep(std::time::Duration::from_millis(50));
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    })?;
    Ok(())
}
