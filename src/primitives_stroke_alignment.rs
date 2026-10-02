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
    loop {
        target.clear(ColorType::BLACK)?;
        let tbl = {
            let style_base = eg::primitives::PrimitiveStyleBuilder::new()
                .stroke_color(ColorType::CYAN).stroke_width(15).build();
            [
                ("Inside", eg::primitives::PrimitiveStyleBuilder::from(&style_base)
                    .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build(),),
                ("Center\n(Default)", eg::primitives::PrimitiveStyleBuilder::from(&style_base)
                    .stroke_alignment(eg::primitives::StrokeAlignment::Center).build(),),
                ("Outside", eg::primitives::PrimitiveStyleBuilder::from(&style_base)
                    .stroke_alignment(eg::primitives::StrokeAlignment::Outside).build(),),
            ]
        };
        let style_shape_outline = eg::primitives::PrimitiveStyleBuilder::new()
            .stroke_color(ColorType::RED).stroke_width(1).build();
        let character_style = eg::mono_font::MonoTextStyleBuilder::new()
            .font(&eg_fonts::FONT_7X13).text_color(ColorType::WHITE).build();
        let mut position = Point::zero() + Size::new(0, PADDING + 16);
        for (label, style) in &tbl {
            let style = *style;
            eg::text::Text::new(label, Point::new(position.x, position.y - 30), character_style).draw(target)?;
            draw_primitives(target, position, style)?;
            if show_shape_outline {
                draw_primitives(target, position, style_shape_outline)?;
            }
            position += Size::new(SIZE + PADDING, 0);
        }
        let text_style = eg::text::TextStyleBuilder::new()
            .baseline(eg::text::Baseline::Bottom).alignment(eg::text::Alignment::Center).build();
        eg::text::Text::with_text_style("Click to toggle shape outline",
            Point::new(bbox.center().x, bbox.size.height as i32), character_style, text_style).draw(target)?;
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
