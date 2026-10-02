//! # Example: Stroke alignment
//!
//! This example demonstrates the different stroke alignments available for primitives.
//!
//! The stroke alignment property only applies to closed shapes and is currently
//! not supported for triangles.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii as eg_fonts;
use embedded_hal as hal;

pub enum Event {
    None,
    Quit,
    MouseButtonDown,
}

const SIZE: u32 = 50;
const PADDING: u32 = 32;

fn draw_primitives<DrawTarget, ColorType>(display: &mut DrawTarget, position: Point,
    style: eg::primitives::PrimitiveStyle<ColorType>) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let row_offset = Size::new(0, SIZE + PADDING);

    eg::primitives::Rectangle::new(position, Size::new(SIZE, SIZE))
        .into_styled(style)
        .draw(display)?;

    eg::primitives::Circle::new(position + row_offset, SIZE)
        .into_styled(style)
        .draw(display)?;

    eg::primitives::Ellipse::new(position + row_offset * 2, Size::new(SIZE, SIZE + 16))
        .into_styled(style)
        .draw(display)
}

pub fn run<DrawTarget, ColorType, DelayNs, FuncInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut func_interval: FuncInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FuncInterval: FnMut(&mut DrawTarget) -> Event,
{
    let mut show_shape_outline = true;
    loop {
        target.clear(ColorType::BLACK)?;

        let style_base = eg::primitives::PrimitiveStyleBuilder::new()
            .stroke_color(ColorType::CYAN).stroke_width(15).build();

        let style_stroke_inside = eg::primitives::PrimitiveStyleBuilder::from(&style_base)
            .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build();

        let style_stroke_center = eg::primitives::PrimitiveStyleBuilder::from(&style_base)
            .stroke_alignment(eg::primitives::StrokeAlignment::Center).build();

        let style_stroke_outside = eg::primitives::PrimitiveStyleBuilder::from(&style_base)
            .stroke_alignment(eg::primitives::StrokeAlignment::Outside).build();

        let style_shape_outline = eg::primitives::PrimitiveStyleBuilder::new()
            .stroke_color(ColorType::RED).stroke_width(1).build();

        let text_offset = Point::new(0, 8) + Size::new(PADDING, 10);
        let text_style = eg::mono_font::MonoTextStyle::new(&eg_fonts::FONT_7X13, ColorType::WHITE);
        let column_offset = Size::new(SIZE + PADDING, 0);

        let mut position = Point::zero() + Size::new(0, PADDING + 16);
        for &style in [style_stroke_inside, style_stroke_center, style_stroke_outside].iter() {
            draw_primitives(target, position, style)?;

            if show_shape_outline {
                draw_primitives(target, position, style_shape_outline)?;
            }

            position += Size::new(SIZE + PADDING, 0);
        }

        eg::text::Text::new("Inside", text_offset, text_style).draw(target)?;

        eg::text::Text::new("Center\n(Default)", text_offset + column_offset, text_style).draw(target)?;

        eg::text::Text::new("Outside", text_offset + column_offset * 2, text_style).draw(target)?;

        eg::text::Text::new("Click to toggle shape outline",
            Point::new(PADDING as i32, 300), text_style).draw(target)?;
        match func_interval(target) {
            Event::None => {},
            Event::Quit => break,
            Event::MouseButtonDown => {
                show_shape_outline = !show_shape_outline;
            }
        }
        delay.delay_ms(20);
    }
    Ok(())
}

/*
fn main() -> Result<(), Infallible> {
    let mut show_shape_outline = true;

    'main_loop: loop {
        update(&mut display, show_shape_outline)?;
        window.update(&display);

        'event_loop: loop {
            for event in window.events() {
                match event {
                    SimulatorEvent::Quit => break 'main_loop,
                    SimulatorEvent::MouseButtonDown { .. } => {
                        show_shape_outline = !show_shape_outline;
                        break 'event_loop;
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    }

    Ok(())
}
*/
