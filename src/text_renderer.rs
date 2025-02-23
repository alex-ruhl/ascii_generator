use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontArc, PxScale};

pub fn render_text_lines(lines: &[String], font_size: u32) -> RgbaImage {
    let font = crate::font_loader::load_system_font();

    let line_height = font_size;
    let total_height = lines.len() as u32 * line_height;
    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(1);
    let width = max_line_length as u32 * (font_size);

    let mut image: RgbaImage = ImageBuffer::from_pixel(width, total_height, Rgba([255, 255, 255, 255]));

    let mut y_offset = 0 as i32;
    let scale = PxScale::from(font_size as f32);

    for line in lines {
        if line.trim().is_empty() {
            y_offset += line_height as i32;
            continue;
        }
        render_text_with_spaces(line, 0, y_offset, scale, &font, font_size, &mut image);
        y_offset += line_height as i32;
    }
    return image;
}

fn render_text_with_spaces(line: &str, x: i32, y: i32, scale: PxScale, font: &FontArc, font_size: u32, image: &mut RgbaImage) {
    let mut current_x = x;
    for c in line.chars() {
        if c != ' ' {
            draw_text_mut(image, Rgba([0, 0, 0, 1]), current_x, y, scale, font, &c.to_string());
        }
        current_x += font_size as i32;
    }
}