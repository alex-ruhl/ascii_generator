use image::imageops;
use imageproc::edges::canny;

pub fn convert_image_to_ascii(image_path: std::path::PathBuf, threshold: f32) -> Vec<String> {
    let img = image::open(image_path).expect("Error when opening the image");
    let blured_img = img.blur(0.5);

    let grayscale_img = imageops::grayscale(&blured_img);
    let canny_img = canny(&grayscale_img, threshold, 90.0);

    let ascii_scale = r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#;

    let mut ascii_art = Vec::new();
    for y in 0..canny_img.height() {
        let mut line = String::new();
        for x in 0..canny_img.width() {
            let mut luma = canny_img.get_pixel(x, y)[0];
            if luma == 0 {
                luma = grayscale_img.get_pixel(x, y)[0];
            } else {
                luma = 0;
            }
            let intensity = (luma as usize * (ascii_scale.len() - 1)) / 255;
            let char = ascii_scale
                            .chars()
                            .nth(intensity)
                            .unwrap();
            line.push(char);
        }
        ascii_art.push(line);
    }
    ascii_art
}
