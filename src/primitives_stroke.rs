//! # Example: Primitive stroke styles
//!
//! This example demonstrates the different stroke styles available for primitives.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

const PADDING: i32 = 16;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor + WebColors,
{
    let mut position = Point::new(10, 10);
    draw_primitives(&mut target.translated(position), 1)?;
    position.y += 64 + PADDING;
    draw_primitives(&mut target.translated(position), 3)?;
    position.y += 64 + PADDING;
    draw_primitives(&mut target.translated(position), 6)?;
    Ok(())
}

fn draw_primitives<DrawTarget, ColorType>(target: &mut DrawTarget, w: u32) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor + WebColors,
{
    const SIZE: u32 = 34;
    let mut position = Point::new(0, 0);
    let style_builder = eg::primitives::PrimitiveStyleBuilder::new().stroke_width(w);

    let style = style_builder.stroke_color(ColorType::CSS_ORANGE_RED).build();
    eg::primitives::Triangle::new(
            Point::new(0, SIZE as i32), Point::new(SIZE as i32, 0), Point::new(SIZE as i32, SIZE as i32))
        .translate(position).into_styled(style).draw(target)?;
    position.x += SIZE as i32 + PADDING;

    let style = style_builder.stroke_color(ColorType::CSS_GOLD).build();
    eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(SIZE, SIZE))
        .translate(position).into_styled(style).draw(target)?;
    position.x += SIZE as i32 + PADDING;

    let style = style_builder.stroke_color(ColorType::CSS_SEA_GREEN).build();
    eg::primitives::Line::new(Point::new(0, 0), Point::new(SIZE as i32, SIZE as i32))
        .translate(position).into_styled(style).draw(target)?;
    position.x += SIZE as i32 + PADDING;

    let style = style_builder.stroke_color(ColorType::CSS_TEAL).build();
    eg::primitives::Circle::new(Point::new(0, 0), SIZE)
        .translate(position).into_styled(style).draw(target)?;
    position.x += SIZE as i32 + PADDING;

    let style = style_builder.stroke_color(ColorType::CSS_STEEL_BLUE).build();
    eg::primitives::RoundedRectangle::new(
            eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(SIZE, SIZE)),
            eg::primitives::CornerRadii::new(Size::new(SIZE / 4, SIZE / 4)))
        .translate(position).into_styled(style).draw(target)?;
    position.x += SIZE as i32 + PADDING;

    let style = style_builder.stroke_color(ColorType::CSS_FUCHSIA).build();
    eg::primitives::Ellipse::new(Point::new(0, 0), Size::new(SIZE * 3 / 2, SIZE))
        .translate(position).into_styled(style).draw(target)
}
