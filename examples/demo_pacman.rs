use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples::demo_pacman;

mod thread_delay;

fn main() -> Result<(), std::convert::Infallible> {
    use eg::prelude::*;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(4).build();
    let mut window = eg_sim::Window::new("Pacman", &output_settings);
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb565>::new(Size::new(65, 65));
    demo_pacman::run(&mut target, thread_delay::ThreadDelay, |target| {
        window.update(target);
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    })?;
    Ok(())
}
