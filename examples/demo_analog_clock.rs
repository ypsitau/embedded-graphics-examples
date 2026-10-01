use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;
use embedded_hal as hal;
use embedded_graphics_examples::demo_analog_clock;

type ColorType = eg::pixelcolor::BinaryColor;

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    struct ThreadDelay;
    impl hal::delay::DelayNs for ThreadDelay {
        fn delay_ns(&mut self, ns: u32) {
            std::thread::sleep(std::time::Duration::from_nanos(u64::from(ns)));
        }
    }
    let mut target = eg_sim::SimulatorDisplay::<ColorType>::new(Size::new(256, 256));
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    let mut window = eg_sim::Window::new("Clock", &output_settings);
    demo_analog_clock::run(&mut target, ThreadDelay, |target| {
        window.update(target);
        window.events().any(|e| e == eg_sim::SimulatorEvent::Quit)
    })
}
