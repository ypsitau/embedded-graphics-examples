//! # Example: Multiline text
//!
//! Exercise the font renderer to demonstrate rendering of multiline text

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii as eg_fonts;

pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor,
{
    let bbox = target.bounding_box();
    let str_text = indoc::indoc! {"
    Far far away, behind the word mountains, far from
    the countries Vokalia and Consonantia, there live
    the blind texts. Separated they live in
    Bookmarksgrove right at the coast of the Semantics,
    a large language ocean. A small river named Duden
    flows by their place and supplies it with the
    necessary regelialia. It is a paradisematic country,
    in which roasted parts of sentences fly into your
    mouth. Even the all-powerful Pointing has no control
    about the blind texts it is an almost unorthographic
    life One day however a small line of blind text by
    the name of Lorem Ipsum decided to leave for the far
    World of Grammar. The Big Oxmox advised her not to do
    so, because there were thousands of bad Commas, wild
    Question Marks and devious Semikoli, but the Little
    Blind Text didn't listen. She packed her seven
    versalia, put her initial into the belt and made
    herself on the way.
    "};
    target.clear(ColorType::BLACK)?;
    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg_fonts::FONT_6X13).text_color(ColorType::WHITE).background_color(ColorType::BLACK)
        .build();
    let text_style = eg::text::TextStyleBuilder::new()
        .baseline(eg::text::Baseline::Top).alignment(eg::text::Alignment::Left).build();
    eg::text::Text::with_text_style(str_text, bbox.top_left, character_style, text_style).draw(target)?;
    Ok(())
}
