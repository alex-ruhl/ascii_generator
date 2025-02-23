# ASCII Generator

## ✨ Description

This tool converts images into ASCII art and saves them as PNG files. It provides a simple CLI interface to control image processing and output.

## 🛠 Installation

Requirements:

- Rust & Cargo installed

```sh
# Clone the repository
git clone https://github.com/your-repo/ascii_converter_cli.git
cd ascii_converter_cli

# Compile
cargo build --release

# The CLI tool is available at ./target/release/ascii_converter
```

## 🔎 Usage

The tool offers several options that can be configured via the command line:

```sh
./ascii_converter -i input.jpg -o output.png --font-size 4 --threshold 15.0
```

### **Available Parameters:**

| Parameter        | Description                                    | Default Value |
| --------------- | --------------------------------------------- | ------------ |
| `-i`, `--input`  | Path to the input image                       | Required     |
| `-o`, `--output` | Path to the output PNG file                   | ascii_art.png  |
| `--font-size`    | Font size for the ASCII image                 | 4            |
| `--threshold`    | Canny edge detection lower threshold          | 15.0         |
| `--help`         | Displays the help menu                        |              |

## 🚀 Example

```sh
./ascii_converter -i ./examples/swans.jpeg -o ./examples/ascii_art.png --font-size 6 --threshold 20.0
```
This will generate an ASCII art image with a font size of 6px and save it as `ascii_art.png`.

### Input image:
![Example input image](./examples/swans.jpg)

### Output image:
![Example output image](./examples/ascii_art.png)
