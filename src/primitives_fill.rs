//! # Example: Primitive fill styles
//!
//! This example demonstrates the different fill and stroke styles available for primitives.

use embedded_graphics as eg;
use embedded_graphics::prelude::*;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor
{
    const SLOT_WIDTH: i32 = 110;
    const SLOT_HEIGHT: i32 = 110;
    let style1 = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::MAGENTA).stroke_width(1).build();
    let style2 = eg::primitives::PrimitiveStyleBuilder::new()
        .fill_color(ColorType::YELLOW).build();
    let style3 = eg::primitives::PrimitiveStyleBuilder::new()
        .stroke_color(ColorType::RED).stroke_width(1)
        .fill_color(ColorType::GREEN).build();
    let styles = [style1, style2, style3];
    let mut offset = Point::new(8, 8);
    target.clear(ColorType::BLACK)?;
    for style in &styles {
        let style = *style;
        eg::primitives::Circle::new(Point::new(0, 0), 65)
            .translate(offset).into_styled(style).draw(target)?;
        eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(64, 64))
            .translate(offset + Point::new(SLOT_WIDTH, 0)).into_styled(style).draw(target)?;
        eg::primitives::Triangle::new(Point::new(33, 0), Point::new(0, 64), Point::new(64, 64))
            .translate(offset + Point::new(SLOT_WIDTH * 2, 0)).into_styled(style).draw(target)?;
        eg::primitives::Ellipse::new(Point::new(0, 0), Size::new(80, 65))
            .translate(offset + Point::new(0, SLOT_HEIGHT)).into_styled(style).draw(target)?;
        eg::primitives::RoundedRectangle::new(
            eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(64, 64)),
            eg::primitives::CornerRadii::new(Size::new(16, 16)),
        ).translate(offset + Point::new(SLOT_WIDTH, SLOT_HEIGHT)).into_styled(style).draw(target)?;
        offset += Point::new(12, 12);
    }
    Ok(())
}
