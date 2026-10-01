use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples as examples;

fn main() -> Result<(), core::convert::Infallible> {
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::BinaryColor>::new(Size::new(512, 128));
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(1).build();
    let mut window = eg_sim::Window::new("text_extended_characters", &output_settings);
    examples::text_extended_characters::show(&mut target)?;
    window.show_static(&target);
    Ok(())
}
