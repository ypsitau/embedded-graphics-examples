#![no_std]

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

use esp_hal as esp;

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
    use esp::gpio;
    let p = esp::init(esp::Config::default());
    let timg0 = esp::timer::timg::TimerGroup::new(p.TIMG0);
    let software_interrupt_control =
        esp::interrupt::software::SoftwareInterruptControl::new(p.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt_control.software_interrupt0);
    let ((pin_i2c0_scl, pin_i2c0_sda), (pin_spi2_sck, pin_spi2_mosi),
            (pin_display_reset, pin_display_dc, pin_display_cs, pin_display_bl), pin_sw) =
        ((p.GPIO22, p.GPIO23), (p.GPIO32, p.GPIO33), (p.GPIO17, p.GPIO5, p.GPIO18, p.GPIO19), p.GPIO26);
        //((p.GPIO21, p.GPIO47), (p.GPIO38, p.GPIO39), (p.GPIO40, p.GPIO41, p.GPIO42, p.GPIO2), p.GPIO4);
    let mutex_i2c0 = {
        type I2c = esp::i2c::master::I2c<'static, esp::Blocking>;
        let config = esp::i2c::master::Config::default().with_frequency(esp::time::Rate::from_khz(400));
        static STATIC_CELL: StaticCell<BlockingMutexCriticalSection<RefCell<I2c>>> = StaticCell::new();
        STATIC_CELL.init(BlockingMutexCriticalSection::new(RefCell::new(
            I2c::new(p.I2C0, config).unwrap().with_scl(pin_i2c0_scl).with_sda(pin_i2c0_sda))))
    };
    let mutex_spi2 = {
        type Spi = esp::spi::master::Spi<'static, esp::Blocking>;
        static STATIC_CELL: StaticCell<BlockingMutexCriticalSection<RefCell<Spi>>> = StaticCell::new();
        STATIC_CELL.init(BlockingMutexCriticalSection::new(RefCell::new(
            Spi::new(p.SPI2, Default::default()).unwrap().with_sck(pin_spi2_sck).with_mosi(pin_spi2_mosi))))
    };
    let pin_display_reset = gpio::Output::new(pin_display_reset, gpio::Level::High, gpio::OutputConfig::default());
    let pin_display_dc = gpio::Output::new(pin_display_dc, gpio::Level::Low, gpio::OutputConfig::default());
    let pin_display_cs = gpio::Output::new(pin_display_cs, gpio::Level::High, gpio::OutputConfig::default());
    let pin_display_bl = gpio::Output::new(pin_display_bl, gpio::Level::High, gpio::OutputConfig::default());
    let pin_sw = gpio::Input::new(pin_sw, gpio::InputConfig::default().with_pull(gpio::Pull::Up));
    let mipidsi_spi_interface = {
        let spi_dev = {
            let config = esp::spi::master::Config::default()
                .with_frequency(esp::time::Rate::from_mhz(8))
                .with_mode(esp::spi::Mode::_3);
            emb::hal::shared_bus::blocking::spi::SpiDeviceWithConfig::new(mutex_spi2, pin_display_cs, config)
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
    let i2c_devs = core::array::from_fn(|_| emb::hal::shared_bus::blocking::i2c::I2cDevice::new(mutex_i2c0));
    BoardWithMipidsi { display, pin_display_bl, pin_sw, i2c_devs, }
}
