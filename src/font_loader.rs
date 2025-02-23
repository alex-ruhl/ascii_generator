use rusttype::Font;
use ab_glyph::FontArc;

pub fn load_system_font() -> FontArc {
    let font_data = include_bytes!("../font/Arial.ttf");
    Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    FontArc::try_from_vec(font_data.to_vec()).expect("Invalid font format")
}