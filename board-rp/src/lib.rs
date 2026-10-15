#![no_std]

pub mod ds3231;
pub mod prelude {
    pub use super::Flushable as _;
}
mod emb {
    pub use embassy_embedded_hal as hal;
    pub use embassy_sync as sync;
    pub use embassy_time as time;
}
use embedded_hal_1 as hal;
use embedded_hal_async as hal_async;
use core::cell::RefCell;
use static_cell::StaticCell;
use embedded_graphics as eg;

use embassy_rp as rp;

pub type BlockingMutexNoop<T> =
    emb::sync::blocking_mutex::Mutex<emb::sync::blocking_mutex::raw::NoopRawMutex, T>;

#[allow(dead_code)]
pub struct BoardWithMipidsi<const N_I2C_DEVS: usize, Display, I2c, OutputPin, InputPin> {
    pub display: Display,
    pub pin_display_bl: OutputPin,
    pub pin_sw: InputPin,
    pub i2c_devs: [I2c; N_I2C_DEVS],
}

#[allow(dead_code)]
pub fn init_with_mipidsi<'d, const N_I2C_DEVS: usize, DisplayModel>(display_model: DisplayModel, size: (u16, u16), orientation: mipidsi::options::Orientation,
    color_order: mipidsi::options::ColorOrder, color_inversion: mipidsi::options::ColorInversion)
    -> BoardWithMipidsi<
        N_I2C_DEVS,
        impl eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb565, Error: core::fmt::Debug>,
        impl hal::i2c::I2c,
        impl hal::digital::OutputPin,
        impl hal::digital::InputPin + hal_async::digital::Wait,
    >
where
    DisplayModel: mipidsi::models::Model<ColorFormat = eg::pixelcolor::Rgb565>,
{
    let p = rp::init(Default::default());
    let mutex_i2c1 = {
        type I2c1 = rp::i2c::I2c<'static, rp::peripherals::I2C1, rp::i2c::Blocking>;
        let (pin_sda, pin_scl) = (p.PIN_14, p.PIN_15);
        let mut config = rp::i2c::Config::default();
        config.frequency = 400_000;
        static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<I2c1>>> = StaticCell::new();
        STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
            I2c1::new_blocking(p.I2C1, pin_scl, pin_sda, config))))
    };
    let mutex_spi0 = {
        type Spi0 = rp::spi::Spi<'static, rp::peripherals::SPI0, rp::spi::Blocking>;
        let (pin_clk, pin_mosi) = (p.PIN_6, p.PIN_7);
        static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<Spi0>>> = StaticCell::new();
        STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
            Spi0::new_blocking_txonly(p.SPI0, pin_clk, pin_mosi, Default::default()))))
    };
    let pin_display_reset = rp::gpio::Output::new(p.PIN_10, rp::gpio::Level::Low);
    let pin_display_dc = rp::gpio::Output::new(p.PIN_11, rp::gpio::Level::Low);
    let pin_display_cs = rp::gpio::Output::new(p.PIN_12, rp::gpio::Level::High);
    let pin_display_bl = rp::gpio::Output::new(p.PIN_13, rp::gpio::Level::High);
    let pin_sw = rp::gpio::Input::new(p.PIN_16, rp::gpio::Pull::Up);
    let mipidsi_spi_interface = {
        let spi_dev = {
            let mut config = rp::spi::Config::default();
            config.frequency = 64_000_000;
            config.phase = rp::spi::Phase::CaptureOnSecondTransition;
            config.polarity = rp::spi::Polarity::IdleHigh;
            emb::hal::shared_bus::blocking::spi::SpiDeviceWithConfig::new(mutex_spi0, pin_display_cs, config)
        };
        let spi_buf = {
            const SPI_BUF_SIZE: usize = 320;
            static STATIC_CELL: StaticCell<[u8; SPI_BUF_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0u8; SPI_BUF_SIZE])
        };
        mipidsi::interface::SpiInterface::new(spi_dev, pin_display_dc, spi_buf)
    };
    let display = mipidsi::Builder::new(display_model, mipidsi_spi_interface)
        .display_size(size.0, size.1).reset_pin(pin_display_reset)
        .orientation(orientation).color_order(color_order).invert_colors(color_inversion)
        .init(&mut emb::time::Delay).unwrap();
    let i2c_devs = core::array::from_fn(|_| emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c1));
    BoardWithMipidsi { display, pin_display_bl, pin_sw, i2c_devs, }
}

#[allow(dead_code)]
pub trait Flushable {
    type Error: core::fmt::Debug;
    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<DI, SIZE> Flushable for ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>
where
    DI: ssd1306::prelude::WriteOnlyDataCommand,
    SIZE: ssd1306::size::DisplaySize,
{
    type Error = <Self as ssd1306::prelude::DisplayConfig>::Error;
    fn flush(&mut self) -> Result<(), Self::Error> {
        ssd1306::Ssd1306::flush(self)
    }
}

#[allow(dead_code)]
pub struct BoardWithSSD1306<const N_I2C_DEVS: usize, Display, I2c, InputPin> {
    pub display: Display,
    pub pin_sw: InputPin,
    pub i2c_devs: [I2c; N_I2C_DEVS],
}

#[allow(dead_code)]
pub fn init_with_ssd1306<'d, const N_I2C_DEVS: usize>(rotation: ssd1306::rotation::DisplayRotation)
    -> BoardWithSSD1306<
        N_I2C_DEVS,
        impl eg::draw_target::DrawTarget<Color = eg::pixelcolor::BinaryColor, Error: core::fmt::Debug> + Flushable,
        impl hal::i2c::I2c,
        impl hal::digital::InputPin + hal_async::digital::Wait,
    >
{
    use ssd1306::prelude::*;
    let p = rp::init(Default::default());
    let mutex_i2c1 = {
        let (pin_sda, pin_scl) = (p.PIN_14, p.PIN_15);
        let mut config = rp::i2c::Config::default();
        config.frequency = 400_000;
        type I2c1 = rp::i2c::I2c<'static, rp::peripherals::I2C1, rp::i2c::Blocking>;
        static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<I2c1>>> = StaticCell::new();
        STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
            I2c1::new_blocking(p.I2C1, pin_scl, pin_sda, config))))
    };
    let pin_sw = rp::gpio::Input::new(p.PIN_16, rp::gpio::Pull::Up);
    let i2c_devs = core::array::from_fn(|_| emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c1));
    let mut display = {
        let i2c_dev = emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c1);
        let interface = ssd1306::I2CDisplayInterface::new(i2c_dev);
        let size = ssd1306::size::DisplaySize128x64;
        ssd1306::Ssd1306::new(interface, size, rotation).into_buffered_graphics_mode()
    };
    display.init().unwrap();
    BoardWithSSD1306 { display, pin_sw, i2c_devs, }
}
