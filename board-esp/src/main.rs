//! SPI SSD1306 setup for a classic ESP32 DevKit.
#![no_std]
#![no_main]

use core::cell::RefCell;
mod emb {
    pub use embassy_embedded_hal as hal;
    pub use embassy_sync as sync;
    pub use embassy_time as time;
}
use static_cell::StaticCell;

use esp_backtrace as _;
use esp_hal as esp;

esp_bootloader_esp_idf::esp_app_desc!();

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::iso_8859_1 as eg_fonts;

pub type BlockingMutexCriticalSection<T> =
    emb::sync::blocking_mutex::Mutex<emb::sync::blocking_mutex::raw::CriticalSectionRawMutex, T>;

#[esp_rtos::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let display_model = mipidsi::models::ST7789;
    let orientation = mipidsi::options::Orientation::new();

    let (spi_display, pin_display_dc, pin_display_reset) = {
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
        use esp::gpio;
        let pin_display_reset = gpio::Output::new(p.GPIO4, gpio::Level::High, gpio::OutputConfig::default());
        let pin_display_dc = gpio::Output::new(p.GPIO16, gpio::Level::Low, gpio::OutputConfig::default());
        let pin_display_cs = gpio::Output::new(p.GPIO17, gpio::Level::High, gpio::OutputConfig::default());
        let _pin_display_bl = gpio::Output::new(p.GPIO5, gpio::Level::High, gpio::OutputConfig::default());
        let spi_display = emb::hal::shared_bus::blocking::spi::SpiDevice::new(mutex_spi, pin_display_cs);
        (spi_display, pin_display_dc, pin_display_reset)
    };
    let mut display = {
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
    show(&mut display).unwrap();
    core::future::pending::<()>().await;
}

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    const SIZE_OBJ: i32 = 64;
    let mut offset = Point::new(10, 14);

    target.clear(ColorType::BLACK)?;

    // Draw a 3px wide outline around the target.
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(3)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build();
    target.bounding_box().into_styled(style).draw(target)?;

    // Draw a triangle.
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(1).build();
    eg::primitives::Triangle::new(Point::new(0, SIZE_OBJ), Point::new(SIZE_OBJ, SIZE_OBJ),
        Point::new(SIZE_OBJ / 2, 0)).translate_mut(offset).into_styled(style).draw(target)?;
    offset.x += SIZE_OBJ + 4;

    // Draw a filled square
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .fill_color(ColorType::WHITE).build();
    eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(SIZE_OBJ as u32, SIZE_OBJ as u32))
        .translate_mut(offset).into_styled(style).draw(target)?;
    offset.x += SIZE_OBJ + 4;

    // Draw a circle with a 3px wide stroke.
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(3).build();
    eg::primitives::Circle::new(Point::new(0, 0), SIZE_OBJ as u32)
        .translate_mut(offset).into_styled(style).draw(target)?;

    // Draw centered text.
    let text = "embedded-graphics";
    let position = target.bounding_box().center() + Point::new(0, 15);
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg_fonts::FONT_10X20).text_color(ColorType::WHITE).build();
    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).build();
    eg::text::Text::with_text_style(text, position, character_style, text_style).draw(target)?;

    Ok(())
}
