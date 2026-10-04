mod emb {
    pub use embassy_embedded_hal as hal;
    pub use embassy_sync as sync;
    pub use embassy_time as time;
}
use embedded_hal_1 as hal;

use core::cell::RefCell;

use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use embassy_rp as rp;

use embedded_graphics as eg;

pub type BlockingMutexNoop<T> =
    emb::sync::blocking_mutex::Mutex<emb::sync::blocking_mutex::raw::NoopRawMutex, T>;

pub fn init_board<'d, DisplayModel>(display_model: DisplayModel, orientation: mipidsi::options::Orientation)
    -> (impl eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb565, Error: core::fmt::Debug>,
        rp::gpio::Output<'d>, impl hal::i2c::I2c)
where
    DisplayModel: mipidsi::models::Model<ColorFormat = eg::pixelcolor::Rgb565>,
{
    let (spi_display, pin_display_reset, pin_display_dc, pin_display_bl, i2c_dev) = {
        let p = rp::init(Default::default());
        let mutex_spi = {
            let pin_clk = p.PIN_10;
            let pin_mosi = p.PIN_11;
            let pin_miso = p.PIN_12;
            type Spi = rp::spi::Spi<'static, rp::peripherals::SPI1, rp::spi::Blocking>;
            static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<Spi>>> = StaticCell::new();
            STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
                Spi::new_blocking(p.SPI1, pin_clk, pin_mosi, pin_miso, Default::default()))))
        };
        let mutex_i2c0 = {
            let pin_sda = p.PIN_16;
            let pin_scl = p.PIN_17;
            let mut config = rp::i2c::Config::default();
            config.frequency = 400_000;
            type I2c0 = rp::i2c::I2c<'static, rp::peripherals::I2C0, rp::i2c::Blocking>;
            static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<I2c0>>> = StaticCell::new();
            STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
                I2c0::new_blocking(p.I2C0, pin_scl, pin_sda, config))))
        };
        let i2c_dev = emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c0);
        let spi_display = {
            let pin_cs = rp::gpio::Output::new(p.PIN_8, rp::gpio::Level::High);
            let mut config = rp::spi::Config::default();
            config.frequency = 64_000_000;
            config.phase = rp::spi::Phase::CaptureOnSecondTransition;
            config.polarity = rp::spi::Polarity::IdleHigh;
            emb::hal::shared_bus::blocking::spi::SpiDeviceWithConfig::new(mutex_spi, pin_cs, config)
        };
        let pin_display_reset = rp::gpio::Output::new(p.PIN_6, rp::gpio::Level::Low);
        let pin_display_dc = rp::gpio::Output::new(p.PIN_7, rp::gpio::Level::Low);
        let pin_display_bl = rp::gpio::Output::new(p.PIN_9, rp::gpio::Level::High);
        (spi_display, pin_display_reset, pin_display_dc, pin_display_bl, i2c_dev)
    };
    let display = {
        use mipidsi::options::{ColorOrder, ColorInversion};
        let display_interface = {
            let spi_buf = {
                const SPI_BUF_SIZE: usize = 320;
                static STATIC_CELL: StaticCell<[u8; SPI_BUF_SIZE]> = StaticCell::new();
                STATIC_CELL.init([0u8; SPI_BUF_SIZE])
            };
            mipidsi::interface::SpiInterface::new(spi_display, pin_display_dc, spi_buf)
        };
        mipidsi::Builder::new(display_model, display_interface)
            .display_size(240, 320)
            .color_order(ColorOrder::Rgb)
            .reset_pin(pin_display_reset)
            .invert_colors(ColorInversion::Inverted)
            .orientation(orientation)
            .init(&mut emb::time::Delay)
            .unwrap()
    };
    (display, pin_display_bl, i2c_dev)
}
