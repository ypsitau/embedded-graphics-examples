use embedded_hal_async as hal_async;

pub struct ThreadDelay;

impl hal_async::delay::DelayNs for ThreadDelay {
    async fn delay_ns(&mut self, ns: u32) {
        tokio::time::sleep(std::time::Duration::from_nanos(u64::from(ns))).await;
    }
}
