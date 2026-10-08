use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics_examples::hello_world_mono as example;

fn main() -> Result<(), core::convert::Infallible> {
    let mut target = eg_sim::SimulatorDisplay::<eg::pixelcolor::BinaryColor>::new(Size::new(128, 64));
    let output_settings = eg_sim::OutputSettingsBuilder::new()
        .theme(eg_sim::BinaryColorTheme::OledWhite).scale(4).build();
    let mut window = eg_sim::Window::new("hello_world_mono", &output_settings);
    example::show(&mut target)?;
    window.show_static(&target);
    Ok(())
}
