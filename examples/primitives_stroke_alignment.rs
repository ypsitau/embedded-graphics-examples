mod common;

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples::primitives_stroke_alignment as example;

fn main() -> Result<(), std::convert::Infallible> {
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb565>::new(Size::new(240, 320));
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    let mut window = eg_sim::Window::new("primitives_stroke_alignment", &output_settings);
    type Event = example::Event;
    example::run(&mut target, common::ThreadDelay, |target| -> Event {
        window.update(target);
        for event in window.events() {
            match event {
                eg_sim::SimulatorEvent::Quit => return Event::Quit,
                eg_sim::SimulatorEvent::MouseButtonDown { .. } => return Event::MouseButtonDown,
                _ => {},
            }
        }
        Event::None
    });
    Ok(())
}
