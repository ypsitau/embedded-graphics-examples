mod common;

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples::demo_analog_clock as example;

#[tokio::main]
async fn main() -> Result<(), core::convert::Infallible> {
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb888>::new(Size::new(240, 320));
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    let mut window = eg_sim::Window::new("demo_analog_clock", &output_settings);
    let fn_get_time = || {
        use chrono::prelude::*;
        let now = chrono::Local::now();
        (now.hour() as u8, now.minute() as u8, now.second() as u8)
    };
    example::run(&mut target, common::ThreadDelay, fn_get_time,
        |target| {
            window.update(target);
            window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
        }).await;
    Ok(())
}
