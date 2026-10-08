mod emb {
    pub use embassy_embedded_hal as hal;
    pub use embassy_sync as sync;
    pub use embassy_time as time;
}
use embassy_embedded_hal::shared_bus::asynch::i2c;
use embedded_hal_1 as hal;
use embedded_hal_async as hal_async;

use core::cell::RefCell;

use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use embassy_rp as rp;

use embedded_graphics as eg;

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
pub fn init_board_with_mipidsi<'d, const N_I2C_DEVS: usize, DisplayModel>(display_model: DisplayModel, orientation: mipidsi::options::Orientation)
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
    let (spi_display, pin_display_reset, pin_display_dc, pin_display_bl, pin_sw, i2c_devs) = {
        let p = rp::init(Default::default());
        let mutex_spi = {
            let pin_clk = p.PIN_14;
            let pin_mosi = p.PIN_15;
            type Spi = rp::spi::Spi<'static, rp::peripherals::SPI1, rp::spi::Blocking>;
            static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<Spi>>> = StaticCell::new();
            STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
                Spi::new_blocking_txonly(p.SPI1, pin_clk, pin_mosi, Default::default()))))
        };
        let mutex_i2c0 = {
            let pin_sda = p.PIN_8;
            let pin_scl = p.PIN_9;
            let mut config = rp::i2c::Config::default();
            config.frequency = 400_000;
            type I2c0 = rp::i2c::I2c<'static, rp::peripherals::I2C0, rp::i2c::Blocking>;
            static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<I2c0>>> = StaticCell::new();
            STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
                I2c0::new_blocking(p.I2C0, pin_scl, pin_sda, config))))
        };
        let spi_display = {
            let pin_cs = rp::gpio::Output::new(p.PIN_12, rp::gpio::Level::High);
            let mut config = rp::spi::Config::default();
            config.frequency = 64_000_000;
            config.phase = rp::spi::Phase::CaptureOnSecondTransition;
            config.polarity = rp::spi::Polarity::IdleHigh;
            emb::hal::shared_bus::blocking::spi::SpiDeviceWithConfig::new(mutex_spi, pin_cs, config)
        };
        let pin_display_reset = rp::gpio::Output::new(p.PIN_10, rp::gpio::Level::Low);
        let pin_display_dc = rp::gpio::Output::new(p.PIN_11, rp::gpio::Level::Low);
        let pin_display_bl = rp::gpio::Output::new(p.PIN_13, rp::gpio::Level::High);
        let pin_sw = rp::gpio::Input::new(p.PIN_16, rp::gpio::Pull::Up);
        let i2c_devs = core::array::from_fn(|_| emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c0));
        (spi_display, pin_display_reset, pin_display_dc, pin_display_bl, pin_sw, i2c_devs)
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
    BoardWithMipidsi { display, pin_display_bl, pin_sw, i2c_devs, }
}

#[allow(dead_code)]
pub struct BoardWithSSD1306<const N_I2C_DEVS: usize, Display, I2c, InputPin> {
    pub display: Display,
    pub pin_sw: InputPin,
    pub i2c_devs: [I2c; N_I2C_DEVS],
}

#[allow(dead_code)]
pub fn init_board_with_ssd1306<'d, const N_I2C_DEVS: usize>(rotation: ssd1306::rotation::DisplayRotation)
    -> BoardWithSSD1306<
        N_I2C_DEVS,
        impl eg::draw_target::DrawTarget<Color = eg::pixelcolor::BinaryColor, Error: core::fmt::Debug>,
        impl hal::i2c::I2c,
        impl hal::digital::InputPin + hal_async::digital::Wait,
    >
{
    let (pin_sw, i2c_ssd1306, i2c_devs) = {
        let p = rp::init(Default::default());
        let mutex_i2c0 = {
            let pin_sda = p.PIN_8;
            let pin_scl = p.PIN_9;
            let mut config = rp::i2c::Config::default();
            config.frequency = 400_000;
            type I2c0 = rp::i2c::I2c<'static, rp::peripherals::I2C0, rp::i2c::Blocking>;
            static STATIC_CELL: StaticCell<BlockingMutexNoop<RefCell<I2c0>>> = StaticCell::new();
            STATIC_CELL.init(BlockingMutexNoop::new(RefCell::new(
                I2c0::new_blocking(p.I2C0, pin_scl, pin_sda, config))))
        };
        let pin_sw = rp::gpio::Input::new(p.PIN_16, rp::gpio::Pull::Up);
        let i2c_ssd1306 = emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c0);
        let i2c_devs = core::array::from_fn(|_| emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c0));
        (pin_sw, i2c_ssd1306, i2c_devs)
    };
    let display = {
        let interface = ssd1306::I2CDisplayInterface::new(i2c_ssd1306);
        let size = ssd1306::size::DisplaySize128x64;
        ssd1306::Ssd1306::new(interface, size, rotation).into_buffered_graphics_mode()
    };
    BoardWithSSD1306 { display, pin_sw, i2c_devs, }
}
