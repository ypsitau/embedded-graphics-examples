use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples as examples;
    
mod thread_delay;

fn main() -> Result<(), core::convert::Infallible> {
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::BinaryColor>::new(Size::new(256, 256));
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    let mut window = eg_sim::Window::new("demo_analog_clock", &output_settings);
    examples::demo_analog_clock::run(&mut target, thread_delay::ThreadDelay, |target| {
        window.update(target);
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    })
}
