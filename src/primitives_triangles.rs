//! # Example: Triangles
//!
//! Shows multiple triangles with different styles.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor + WebColors,
{
    let padding = Point::new(10, 20);
    //let size = target.bounding_box().size.height as i32 - (padding.y * 2);
    let size = 45;
    let offset = size + 5;

    let triangle_up = eg::primitives::Triangle::new(
        Point::new(0, size), Point::new(size / 2, 0), Point::new(size, size)
    ).translate(padding);

    let triangle_down = eg::primitives::Triangle::new(
        Point::new(0, 0), Point::new(size, 0), Point::new(size / 2, size),
    ).translate(padding);

    let mut position = Point::new(0, 0);

    // Inside thick stroke, no fill
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::CSS_SALMON).stroke_width(5)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build();
    triangle_up.translate(position).into_styled(style).draw(target)?;
    position.x += offset;

    // Center stroke alignment with fill
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::CSS_AQUAMARINE).stroke_width(5)
        .fill_color(ColorType::CSS_CADET_BLUE).build();
    triangle_down.translate(position).into_styled(style).draw(target)?;
    position.x += offset;

    // Outside stroke alignment with fill
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::CSS_FIRE_BRICK).stroke_width(4)
        .stroke_alignment(eg::primitives::StrokeAlignment::Outside)
        .fill_color(ColorType::CSS_WHITE_SMOKE).build();
    triangle_up.translate(position).into_styled(style).draw(target)?;
    position.x += offset;

    // Fill only
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .fill_color(ColorType::CSS_CORAL).build();
    triangle_down.translate(position).into_styled(style).draw(target)?;
    position.x += offset;

    // 1px stroke, no fill
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::WHITE).stroke_width(1).build();
    triangle_up.translate(position).into_styled(style).draw(target)?;
    position.x += offset;

    // Really thick stroke with inside alignment
    let style = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::CSS_DARK_TURQUOISE).stroke_width(10)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside).build();
    triangle_down.translate(position).into_styled(style).draw(target)?;
    Ok(())
}
