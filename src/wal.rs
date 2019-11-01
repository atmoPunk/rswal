use crate::backend::Backend;
use crate::color::Color;
use std::path::Path;
use std::process::Command;

pub struct WalBackend {}

impl Backend for WalBackend {
    fn generate_colors(&self, img: &Path) -> Vec<Color> {
        let magick_command = "magick";
        let mut raw_colors: Vec<Color> = Vec::new();
        for i in 0..20 {
            raw_colors = WalBackend::imagemagick(16 + i, img, magick_command);
            if raw_colors.len() > 15 {
                break;
            }
        }

        if raw_colors.is_empty() {
            panic!("Imagemagick couldn't generate a suitable palette.")
        }

        raw_colors
    }
}

impl WalBackend {
    fn imagemagick(color_count: i32, img: &Path, magic_command: &str) -> Vec<Color> {
        let flags = [
            img.to_str().unwrap(),
            "-resize",
            "25%",
            "-colors",
            &color_count.to_string(),
            "-unique-colors",
            "txt:-",
        ];

        let output = Command::new(magic_command)
            .args(&flags)
            .output()
            .expect("failed to execute imagemagick");
        let colors = output.stdout;
        let colors: Vec<Color> = String::from_utf8(colors)
            .expect("Failed to parse colors")
            .lines()
            .skip(1)
            .map(|line| Color::find_color(line))
            .collect();
        colors
    }
}
