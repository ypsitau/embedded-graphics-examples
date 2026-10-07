#![no_std]
#![no_main]
mod emb {
    pub use embassy_executor as executor;
    pub use embassy_futures as futures;
    pub use embassy_time as time;
}
mod common;

use {defmt_rtt as _, panic_probe as _};
use embedded_graphics_examples::primitives_stroke_alignment as example;
use embedded_hal_1::digital::InputPin as _;
use embedded_hal_async::digital::Wait as _;
use embedded_hal_async::delay::DelayNs as _;

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let orientation = mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg0);
    let (mut display, _pin_display_bl, mut pin_sw, _i2c_dev) = common::init_board(mipidsi::models::ST7789, orientation);
    let event = core::cell::Cell::new(example::Event::None);
    let mut debounce_delay = emb::time::Delay;
    let task_sw = async {
        loop {
            pin_sw.wait_for_any_edge().await.unwrap();
            debounce_delay.delay_ms(50).await;
            if pin_sw.is_low().unwrap() {
                event.set(example::Event::MouseButtonDown);
            }
        }
    };
    let task_example = example::run(&mut display, emb::time::Delay, |_| {
        event.replace(example::Event::None)
    });
    emb::futures::join::join(task_sw, task_example).await;
}
