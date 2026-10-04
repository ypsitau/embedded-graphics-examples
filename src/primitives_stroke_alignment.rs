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

pub fn run<DrawTarget, ColorType, DelayNs, FuncInterval>(target: &mut DrawTarget,
    mut delay: DelayNs, mut func_interval: FuncInterval) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
    DelayNs: hal::delay::DelayNs,
    FuncInterval: FnMut(&mut DrawTarget) -> Event,
{
    let bbox = target.bounding_box();
    let mut show_shape_outline = true;
    'running: loop {
        target.clear(ColorType::BLACK)?;
        let tbl = {
            use eg::primitives::StrokeAlignment as SA;
            let style_builder = eg::primitives::PrimitiveStyleBuilder::new()
                .stroke_color(ColorType::CYAN).stroke_width(15);
            [
                ("Inside",              style_builder.stroke_alignment(SA::Inside).build()),
                ("Center\n(Default)",   style_builder.stroke_alignment(SA::Center).build()),
                ("Outside",             style_builder.stroke_alignment(SA::Outside).build()),
            ]
        };
        let style_shape_outline = eg::primitives::PrimitiveStyleBuilder::new()
            .stroke_color(ColorType::RED).stroke_width(1).build();
        let character_style = eg::mono_font::MonoTextStyleBuilder::new()
            .font(&eg_fonts::FONT_7X13).text_color(ColorType::WHITE).build();
        let text_style = eg::text::TextStyleBuilder::new()
            .baseline(eg::text::Baseline::Top).alignment(eg::text::Alignment::Center).build();
        let mut x = 5;
        for (label, style) in &tbl {
            let style = *style;
            let position_label = Point::new(x + SIZE as i32 / 2, 6);
            eg::text::Text::with_text_style(label, position_label, character_style, text_style).draw(target)?;
            let position = Point::new(x, 50);
            draw_primitives(target, position, style)?;
            if show_shape_outline {
                draw_primitives(target, position, style_shape_outline)?;
            }
            x += (SIZE + PADDING) as i32;
        }
        let text_style = eg::text::TextStyleBuilder::new()
            .baseline(eg::text::Baseline::Bottom).alignment(eg::text::Alignment::Center).build();
        eg::text::Text::with_text_style("Click to toggle shape outline",
            Point::new(bbox.center().x, bbox.size.height as i32 - 6), character_style, text_style).draw(target)?;
        'wait_event: loop {
            match func_interval(target) {
                Event::None => {},
                Event::Quit => break 'running,
                Event::MouseButtonDown => {
                    show_shape_outline = !show_shape_outline;
                    break 'wait_event;
                }
            }
            delay.delay_ms(20);
        }
    }
    Ok(())
}

fn draw_primitives<DrawTarget, ColorType>(target: &mut DrawTarget, mut position: Point,
    style: eg::primitives::PrimitiveStyle<ColorType>) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let y_offset = (SIZE + PADDING) as i32; 
    eg::primitives::Rectangle::new(position, Size::new(SIZE, SIZE)).into_styled(style).draw(target)?;
    position.y += y_offset;
    eg::primitives::Circle::new(position, SIZE).into_styled(style).draw(target)?;
    position.y += y_offset;
    eg::primitives::Ellipse::new(position, Size::new(SIZE, SIZE + 16)).into_styled(style).draw(target)
}
