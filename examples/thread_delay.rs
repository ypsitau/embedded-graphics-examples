use embedded_hal as hal;

pub struct ThreadDelay;

impl hal::delay::DelayNs for ThreadDelay {
    fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(std::time::Duration::from_nanos(u64::from(ns)));
    }
}
