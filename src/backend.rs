use crate::brightness::Brightness;
use crate::color::Color;
use std::path::Path;

pub trait Backend {
    fn generate_colors(&self, img: &Path) -> Vec<Color>;
}

pub fn get(img: &Path, light: Brightness, backend: Box<dyn Backend>) -> Vec<Color> {
    let colors: Vec<Color> = backend.generate_colors(img);
    adjust(&colors, light)
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
            if raw_colors[0].r > 16 {
                raw_colors[0] = raw_colors[0].darken(0.4);
            }

            raw_colors[7] = raw_colors[7].blend(Color::from_hex("#EEEEEE"));
            raw_colors[8] = raw_colors[7].darken(0.3);
            raw_colors[15] = raw_colors[15].blend(Color::from_hex("#EEEEEE"));
        }
    }
    raw_colors
}
