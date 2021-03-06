use crate::util::get_max;
use crate::util::get_min;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorOrderBy {
    R,
    G,
    B,
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn from_hex(rgb_hex: &str) -> Self {
        let r = u8::from_str_radix(&rgb_hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&rgb_hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&rgb_hex[5..], 16).unwrap();
        Color { r, g, b }
    }

    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn darken(self, amount: f32) -> Self {
        Color {
            r: (f32::from(self.r) * (1.0 - amount)) as u8,
            g: (f32::from(self.g) * (1.0 - amount)) as u8,
            b: (f32::from(self.b) * (1.0 - amount)) as u8,
        }
    }

    pub fn lighten(self, amount: f32) -> Self {
        Color {
            r: (f32::from(self.r) + f32::from(255 - self.r) * amount) as u8,
            g: (f32::from(self.g) + f32::from(255 - self.g) * amount) as u8,
            b: (f32::from(self.b) + f32::from(255 - self.b) * amount) as u8,
        }
    }

    pub fn blend(self, other: Self) -> Self {
        Color {
            r: (self.r / 2 + other.r / 2),
            g: (self.g / 2 + other.g / 2),
            b: (self.b / 2 + other.b / 2),
        }
    }

    pub fn saturate(self, amount: f32) -> Self {
        let [h, _, l] = self.to_hsl();
        let s = amount;
        Color::from_hsl(h, s, l)
    }

    pub fn to_hsl(self) -> [f32; 3] {
        let r_normalized = f32::from(self.r) / 255.0;
        let g_normalized = f32::from(self.g) / 255.0;
        let b_normalized = f32::from(self.b) / 255.0;
        let max = get_max(r_normalized, g_normalized, b_normalized);
        let min = get_min(r_normalized, g_normalized, b_normalized);
        let error = std::f32::EPSILON;
        let mut h = match max {
            _ if (max - min).abs() < error => 0.0,
            _ if (max - r_normalized).abs() < error => {
                60.0 * (0.0 + (g_normalized - b_normalized) / (max - min))
            }
            _ if (max - g_normalized).abs() < error => {
                60.0 * (2.0 + (b_normalized - r_normalized) / (max - min))
            }
            _ if (max - b_normalized).abs() < error => {
                60.0 * (4.0 + (r_normalized - g_normalized) / (max - min))
            }
            _ => unreachable!(),
        };
        if h < 0.0 {
            h += 360.0;
        }
        let s = match max {
            _ if (max - 0.0).abs() < error => 0.0,
            _ => match min {
                _ if (min - 1.0).abs() < error => 0.0,
                _ => (max - min) / (1.0 - (max + min - 1.0).abs()),
            },
        };
        let l = (max + min) * 0.5;
        [h, s, l]
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_segment = (h / 60.0) as i32;
        let x = chroma * (1.0 - (h_segment % 2 - 1) as f32);
        let (r1, g1, b1) = match h_segment {
            0 => (chroma, x, 0.0),
            1 => (x, chroma, 0.0),
            2 => (0.0, chroma, x),
            3 => (0.0, x, chroma),
            4 => (x, 0.0, chroma),
            5 => (chroma, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };
        let m = l - chroma * 0.5;
        Color {
            r: ((r1 + m) * 255.0) as u8,
            g: ((g1 + m) * 255.0) as u8,
            b: ((b1 + m) * 255.0) as u8,
        }
    }

    pub fn compare_by(p1: Color, p2: Color, order: ColorOrderBy) -> std::cmp::Ordering {
        match order {
            ColorOrderBy::R => {
                if p1.r < p2.r {
                    std::cmp::Ordering::Less
                } else if p1.r == p2.r {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            ColorOrderBy::G => {
                if p1.g < p2.g {
                    std::cmp::Ordering::Less
                } else if p1.g == p2.g {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            ColorOrderBy::B => {
                if p1.b < p2.b {
                    std::cmp::Ordering::Less
                } else if p1.b == p2.b {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        }
    }

    pub fn find_color(line: &str) -> Color {
        let start = line.find('#').unwrap();
        Color::from_hex(&line[start..start + 7])
    }

    pub fn average_color(pixels: &[Color]) -> Color {
        let mut r: u64 = 0;
        let mut g: u64 = 0;
        let mut b: u64 = 0;
        for pixel in pixels.iter() {
            r += u64::from(pixel.r);
            g += u64::from(pixel.g);
            b += u64::from(pixel.b);
        }
        r /= pixels.len() as u64;
        g /= pixels.len() as u64;
        b /= pixels.len() as u64;
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        Color::new(r, g, b)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
