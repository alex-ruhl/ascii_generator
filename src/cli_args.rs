use clap::{Arg, Command};
use std::path::PathBuf;

pub struct CliArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    pub font_size: u32,
    pub threshold: f32,
}

pub fn parse() -> CliArgs {
    let matches = Command::new("ASCII Converter")
        .version("1.0")
        .author("Alex Ruhl")
        .about("Converts images to ASCII art")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Path to the input image")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Path to the ouput image")
            .default_value("ascii_art.png"))
        .arg(Arg::new("font-size")
            .long("font-size")
            .value_name("SIZE")
            .help("Font size for the ASCII image")
            .default_value("4"))
        .arg(Arg::new("threshold")
            .long("threshold")
            .value_name("WERT")
            .help("Canny edge detection lower threshold")
            .default_value("15.0"))
        .get_matches();
    
    CliArgs {
        input: matches.get_one::<String>("input").unwrap().into(),
        output: matches.get_one::<String>("output").unwrap().into(),
        font_size: matches.get_one::<String>("font-size").unwrap().parse().expect("Invalid font size"),
        threshold: matches.get_one::<String>("threshold").unwrap().parse().expect("Invalid Canny edge detection lower threshold"),
    }
}
