use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples::demo_progress_mono as example;

mod util;

#[tokio::main]
async fn main() -> Result<(), std::convert::Infallible> {
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::BinaryColor>::new(Size::new(128, 64));
    let output_settings = eg_sim::OutputSettingsBuilder::new()
        .theme(eg_sim::BinaryColorTheme::OledWhite).scale(4).build();
    let mut window = eg_sim::Window::new("demo_progress_mono", &output_settings);
    example::run(&mut target, util::ThreadDelay, |target| {
        window.update(target);
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    }).await;
    Ok(())
}
