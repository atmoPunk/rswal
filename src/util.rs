use crate::color::Color;
use crate::brightness::Brightness;

pub fn get_max(a: f32, b: f32, c: f32) -> f32 {
    if a > b {
        if a > c {
            a
        } else {
            c
        }
    } else if b > c {
        b
    } else {
        c
    }
}

pub fn get_min(a: f32, b: f32, c: f32) -> f32 {
    if a < b {
        if a < c {
            a
        } else {
            c
        }
    } else if b < c {
        b
    } else {
        c
    }
}

pub fn set_fg(color: Color) -> String {
    format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b)
}

pub fn set_bg(color: Color) -> String {
    format!("\x1b[48;2;{};{};{}m", color.r, color.g, color.b)
}

pub fn generic_adjust(colors: &[Color], light: Brightness) -> Vec<Color> {
    let mut res: Vec<Color> = colors.to_vec();
    match light {
        Brightness::Light => {
            for color in res.iter_mut() {
                *color = color.saturate(0.6);
                *color = color.darken(0.5);
            }

            res[0] = res[0].lighten(0.95);
            res[7] = res[0].darken(0.75);
            res[8] = res[0].darken(0.25);
            res[15] = res[7]; 
        },
        Brightness::Dark => {
            res[0] = res[0].darken(0.8);
            res[7] = res[0].lighten(0.75);
            res[8] = res[0].lighten(0.25);
            res[15] = res[7]; 
        }
    }
    res
}

pub struct Palette {
    background: Color,
    foreground: Color,
    cursor: Color,
    colors: [Color; 16]
}