use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples::demo_analog_clock;

mod thread_delay;

type ColorType = eg::pixelcolor::BinaryColor;

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut target = eg_sim::SimulatorDisplay::<ColorType>::new(Size::new(256, 256));
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    let mut window = eg_sim::Window::new("Clock", &output_settings);
    demo_analog_clock::run(&mut target, thread_delay::ThreadDelay, |target| {
        window.update(target);
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    })
}
