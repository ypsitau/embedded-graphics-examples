//! # Example: Polyline
//!
//! This example draws a crude "heartbeat" shape using the `Polyline` primitive

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

const PADDING: i32 = 8;

fn draw_polyline<DrawTarget>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb888>,
{
    use eg::prelude::*;
    let bbox = target.bounding_box();
    let h = bbox.size.height as i32;
    let line_style = eg::primitives::PrimitiveStyle::with_stroke(eg::pixelcolor::Rgb888::GREEN, 7);
    let points = [
        Point::new(PADDING, h / 2),
        Point::new(50, h / 2),
        Point::new(60, h / 2 - 20),
        Point::new(70, h / 2),
        Point::new(80, h / 2),
        Point::new(90, h / 2 + 10),
        Point::new(100, PADDING),
        Point::new(110, h / 2 + 20),
        Point::new(120, h / 2),
        Point::new(160, h / 2),
    ];

    eg::primitives::Polyline::new(&points)
        .into_styled(line_style)
        .draw(target)?;
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb888>::new(Size::new(176, 170));
    draw_polyline(&mut target)?;
    eg_sim::Window::new("Polyline", &eg_sim::OutputSettings::default()).show_static(&target);
    Ok(())
}
