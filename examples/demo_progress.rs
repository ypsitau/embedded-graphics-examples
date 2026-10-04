use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples::demo_progress as example;

mod common;

fn main() -> Result<(), std::convert::Infallible> {
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb565>::new(Size::new(240, 320));
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    let mut window = eg_sim::Window::new("demo_progress", &output_settings);
    example::run(&mut target, common::ThreadDelay, |target| {
        window.update(target);
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    });
    Ok(())
}
