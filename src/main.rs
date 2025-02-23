mod ascii_converter;
mod text_renderer;
mod font_loader;
mod cli_args;

fn main() {
    let args = cli_args::parse();
    println!("Input: {:?}, Output: {:?}, Font-Size: {}, Canny lower threshold: {}", 
        args.input, args.output, args.font_size, args.threshold);
    
    let ascii_art = ascii_converter::convert_image_to_ascii(args.input, args.threshold);
    let image = text_renderer::render_text_lines(&ascii_art, args.font_size);
    
    image.save(args.output).expect("Error saving the image");
    println!("ASCII-Art successfully created!");
}