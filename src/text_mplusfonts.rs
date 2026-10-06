//! # Example: Fonts
//!
//! Demonstrate some of the available builtin fonts. A full list of fonts can be found in the
//! [embedded-graphics documentation](https://docs.rs/embedded-graphics).

use embedded_graphics as eg;
use embedded_graphics::prelude::*;
use embedded_text as embtext;

#[mplusfonts::strings]
pub fn show<DrawTarget, ColorType>(target: &mut DrawTarget) -> Result<(), DrawTarget::Error>
where
    DrawTarget: eg::draw_target::DrawTarget<Color = ColorType>,
    ColorType: eg::pixelcolor::RgbColor + Default
        + mplusfonts::color::Invert + mplusfonts::color::Screen + mplusfonts::color::WeightedAvg,
    mplusfonts::color::Colormap<ColorType, 16>: mplusfonts::color::Linear<ColorType>,
{
    target.clear(ColorType::BLACK)?;
    #[strings::emit]
    let bitmap_font = mplusfonts::mplus!(code(115), NORMAL, code_line_height(16), true, 1, 4, '0'..='9');
    let character_style = mplusfonts::style::BitmapFontStyleBuilder::new()
        .font(&bitmap_font).text_color(ColorType::WHITE).build();
    let textbox_style = embtext::style::TextBoxStyleBuilder::new()
        .alignment(embtext::alignment::HorizontalAlignment::Left)
        .vertical_alignment(embtext::alignment::VerticalAlignment::Top).build();
    let str_text = indoc::indoc! {"
    親譲りの無鉄砲で小供の時から損ばかりしている。小学校に居る時分学校の二階から飛び降りて一週間ほど腰を抜かした事がある。\
    なぜそんな無闇をしたと聞く人があるかも知れぬ。別段深い理由でもない。\
    新築の二階から首を出していたら、同級生の一人が冗談に、いくら威張っても、そこから飛び降りる事は出来まい。弱虫やーい。と囃したからである。\
    小使に負ぶさって帰って来た時、おやじが大きな眼をして二階ぐらいから飛び降りて腰を抜かす奴があるかと云ったから、この次は抜かさずに飛んで見せますと答えた。
    親類のものから西洋製のナイフを貰って奇麗な刃を日に翳して、友達に見せていたら、一人が光る事は光るが切れそうもないと云った。\
    切れぬ事があるか、何でも切ってみせると受け合った。そんなら君の指を切ってみろと注文したから、何だ指ぐらいこの通りだと右の手の親指の甲をはすに切り込んだ。\
    幸ナイフが小さいのと、親指の骨が堅かったので、今だに親指は手に付いている。しかし創痕は死ぬまで消えぬ。
    "};
    embtext::TextBox::with_textbox_style(str_text,
        target.bounding_box(), character_style, textbox_style).draw(target)?;
    Ok(())
}
