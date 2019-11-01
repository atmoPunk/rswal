use crate::backend::Backend;
use crate::color::{Color, ColorOrderBy};
use image::GenericImageView;
use std::path::Path;

pub struct MedianCutBackend {}

impl Backend for MedianCutBackend {
    fn generate_colors(&self, img: &Path) -> Vec<Color> {
        let (w, h) = image::image_dimensions(img).expect("Can't read dimensions");
        let (new_w, new_h) = (w / 4, h / 4);
        let img = image::open(img).expect("Can't open image");
        let resized_image = img.resize(new_w, new_h, image::imageops::Gaussian); // check other filters
        get_colors(&resized_image, 16)
    }
}

fn get_ranges(pixels: &[Color]) -> (u8, u8, u8) {
    let Color {
        r: mut min_r,
        g: mut min_g,
        b: mut min_b,
    } = pixels[0];
    let Color {
        r: mut max_r,
        g: mut max_g,
        b: mut max_b,
    } = pixels[0];
    for pixel in pixels.iter().skip(1) {
        min_r = std::cmp::min(min_r, pixel.r);
        min_g = std::cmp::min(min_g, pixel.g);
        min_b = std::cmp::min(min_b, pixel.b);
        max_r = std::cmp::max(max_r, pixel.r);
        max_g = std::cmp::max(max_g, pixel.g);
        max_b = std::cmp::max(max_b, pixel.b);
    }
    (max_r - min_r, max_g - min_g, max_b - min_b)
}

fn get_colors(img: &image::DynamicImage, colors_num: u32) -> Vec<Color> {
    let mut pixels: Vec<Color> = img
        .pixels()
        .map(|(_, _, pix)| Color {
            r: pix[0],
            g: pix[1],
            b: pix[2],
        })
        .collect();
    split_into_buckets(&mut pixels, 1, colors_num)
}

fn split_into_buckets(pixels: &mut [Color], depth: u32, max_depth: u32) -> Vec<Color> {
    let mut result: Vec<Color> = Vec::with_capacity(16usize / depth as usize);
    if depth >= max_depth {
        let color = Color::average_color(pixels);
        result.push(color);
    } else {
        let (r_range, g_range, b_range) = get_ranges(&pixels);
        if r_range >= g_range && r_range >= b_range {
            pixels.sort_by(|&p1, &p2| Color::compare_by(p1, p2, ColorOrderBy::R));
        } else if g_range >= r_range && g_range >= b_range {
            pixels.sort_by(|&p1, &p2| Color::compare_by(p1, p2, ColorOrderBy::G));
        } else {
            pixels.sort_by(|&p1, &p2| Color::compare_by(p1, p2, ColorOrderBy::B));
        }

        let (mut bucket_l, mut bucket_r) = pixels.split_at_mut(pixels.len() / 2);
        result.extend(split_into_buckets(&mut bucket_l, depth * 2, max_depth));
        result.extend(split_into_buckets(&mut bucket_r, depth * 2, max_depth));
    }

    result
}
