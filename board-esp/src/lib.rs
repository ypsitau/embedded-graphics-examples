#![no_std]

mod emb {
    pub use embassy_embedded_hal as hal;
    pub use embassy_sync as sync;
}
use embedded_hal_1 as hal;
use embedded_hal_async as hal_async;
use core::cell::RefCell;
use static_cell::StaticCell;
use esp_hal as esp;
use embedded_graphics as eg;

esp_bootloader_esp_idf::esp_app_desc!();

pub type BlockingMutexCriticalSection<T> =
    emb::sync::blocking_mutex::Mutex<emb::sync::blocking_mutex::raw::CriticalSectionRawMutex, T>;

#[allow(dead_code)]
pub struct BoardWithMipidsi<const N_I2C_DEVS: usize, Display, I2c, OutputPin, InputPin> {
    pub display: Display,
    pub pin_display_bl: OutputPin,
    pub pin_sw: InputPin,
    pub i2c_devs: [I2c; N_I2C_DEVS],
}

#[allow(dead_code)]
pub fn init_with_mipidsi<'d, const N_I2C_DEVS: usize, DisplayModel>(display_model: DisplayModel, orientation: mipidsi::options::Orientation)
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
    let (spi_display, pin_display_dc, pin_display_reset, pin_display_bl, pin_sw, i2c_devs) = {
        let p = esp::init(esp::Config::default());
        let timg0 = esp::timer::timg::TimerGroup::new(p.TIMG0);
        let software_interrupt_control =
            esp::interrupt::software::SoftwareInterruptControl::new(p.SW_INTERRUPT);
        esp_rtos::start(timg0.timer0, software_interrupt_control.software_interrupt0);
        let mutex_spi = {
            let pin_sck = p.GPIO18;
            let pin_mosi = p.GPIO19;
            type Spi = esp::spi::master::Spi<'static, esp::Blocking>;
            let config = esp::spi::master::Config::default().with_frequency(esp::time::Rate::from_mhz(8));
            static STATIC_CELL: StaticCell<BlockingMutexCriticalSection<RefCell<Spi>>> = StaticCell::new();
            STATIC_CELL.init(BlockingMutexCriticalSection::new(RefCell::new(
                Spi::new(p.SPI2, config).unwrap().with_sck(pin_sck).with_mosi(pin_mosi))))
        };
        let mutex_i2c0 = {
            let (pin_sda, pin_scl) = (p.GPIO23, p.GPIO22);
            let config = esp::i2c::master::Config::default()
                .with_frequency(esp::time::Rate::from_khz(400));
            type I2c = esp::i2c::master::I2c<'static, esp::Blocking>;
            static STATIC_CELL: StaticCell<BlockingMutexCriticalSection<RefCell<I2c>>> = StaticCell::new();
            STATIC_CELL.init(BlockingMutexCriticalSection::new(RefCell::new(
                I2c::new(p.I2C0, config).unwrap().with_scl(pin_scl).with_sda(pin_sda))))
        };
        use esp::gpio;
        let pin_display_reset = gpio::Output::new(p.GPIO4, gpio::Level::High, gpio::OutputConfig::default());
        let pin_display_dc = gpio::Output::new(p.GPIO16, gpio::Level::Low, gpio::OutputConfig::default());
        let pin_display_cs = gpio::Output::new(p.GPIO17, gpio::Level::High, gpio::OutputConfig::default());
        let pin_display_bl = gpio::Output::new(p.GPIO5, gpio::Level::High, gpio::OutputConfig::default());
        let pin_sw = gpio::Input::new(p.GPIO21, gpio::InputConfig::default().with_pull(gpio::Pull::Up));
        let i2c_devs = core::array::from_fn(|_| emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c0));
        let spi_display = emb::hal::shared_bus::blocking::spi::SpiDevice::new(mutex_spi, pin_display_cs);
        (spi_display, pin_display_dc, pin_display_reset, pin_display_bl, pin_sw, i2c_devs)
    };
    let display = {
        use mipidsi::options::{ColorInversion, ColorOrder};
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
            .init(&mut esp::delay::Delay::new())
            .unwrap()
    };
    BoardWithMipidsi { display, pin_display_bl, pin_sw, i2c_devs, }
}
