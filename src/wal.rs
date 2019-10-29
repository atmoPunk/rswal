use crate::brightness::Brightness;
use crate::color::Color;
use std::path::Path;
use std::process::Command;

pub fn get(img: &Path, light: Brightness) -> Vec<Color> {
    let colors: Vec<Color> = generate_colors(img);
    adjust(&colors, light)
}

fn find_color(line: &str) -> Color {
    let start = line.find('#').unwrap();
    Color::from_hex(&line[start..start + 7])
}

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
        .map(|line| find_color(line))
        .collect();
    colors
}

fn generate_colors(img: &Path) -> Vec<Color> {
    let magick_command = "magick";
    let mut raw_colors: Vec<Color> = Vec::new();
    for i in 0..20 {
        raw_colors = imagemagick(16 + i, img, magick_command);
        if raw_colors.len() > 15 {
            break;
        }
    }

    if raw_colors.is_empty() {
        panic!("Imagemagick couldn't generate a suitable palette.")
    }

    raw_colors
}

fn adjust(colors: &[Color], light: Brightness) -> Vec<Color> {
    let mut raw_colors = Vec::with_capacity(16);
    raw_colors.push(colors[0]);
    raw_colors.extend_from_slice(&colors[8..16]);
    raw_colors.extend_from_slice(&colors[8..15]);
    match light {
        Brightness::Light => {
            for color in raw_colors.iter_mut() {
                color.saturate(0.5);
            }
            raw_colors[0] = colors.last().unwrap().lighten(0.85);
            raw_colors[7] = colors[0];
            raw_colors[8] = colors.last().unwrap().darken(0.4);
            raw_colors[15] = colors[0];
        }
        Brightness::Dark => {
            if raw_colors[0].r() > 16 {
                raw_colors[0] = raw_colors[0].darken(0.4);
            }

            raw_colors[7] = raw_colors[7].blend(Color::from_hex("#EEEEEE"));
            raw_colors[8] = raw_colors[7].darken(0.3);
            raw_colors[15] = raw_colors[15].blend(Color::from_hex("#EEEEEE"));
        }
    }
    raw_colors
}
