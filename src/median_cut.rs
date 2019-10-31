use image::GenericImageView;
use std::path::Path;
use crate::color::Color;

pub fn get_pallette(img: &Path) -> Vec<Color> {
    let (w, h) = image::image_dimensions(img).expect("Can't read dimensions");
    let (new_w, new_h) = (w / 4, h / 4);
    let img = image::open(img).expect("Can't open image");
    let resized_image = img.resize(new_w, new_h, image::imageops::Gaussian); // check other filters
    get_colors(&resized_image, 16)
}

struct MyPixel {
    r: u8,
    g: u8,
    b: u8,
}

fn get_ranges(pixels: &[MyPixel]) -> (u8, u8, u8) {
    let MyPixel { r: mut min_r, g: mut min_g, b: mut min_b } = pixels[0]; 
    let MyPixel { r: mut max_r, g: mut max_g, b: mut max_b } = pixels[0];
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

enum MyPixelOrderBy {
    R,
    G,
    B,
}

impl MyPixel {
    fn compare_by(p1: &MyPixel, p2: &MyPixel, order: MyPixelOrderBy) -> std::cmp::Ordering {
        match order {
            MyPixelOrderBy::R => {
                if p1.r < p2.r { std::cmp::Ordering::Less }
                else if p1.r == p2.r { std::cmp::Ordering::Equal }
                else { std::cmp::Ordering::Greater }
            },
            MyPixelOrderBy::G => {
                if p1.g < p2.g { std::cmp::Ordering::Less }
                else if p1.g == p2.g { std::cmp::Ordering::Equal }
                else { std::cmp::Ordering::Greater }
            },
            MyPixelOrderBy::B => {
                if p1.b < p2.b { std::cmp::Ordering::Less }
                else if p1.b == p2.b { std::cmp::Ordering::Equal }
                else { std::cmp::Ordering::Greater }
            }
        }
    }
}

fn get_colors(img: &image::DynamicImage, colors_num: u32) -> Vec<Color> {
    let mut pixels: Vec<MyPixel> = img.pixels().map(|(_, _, pix)| MyPixel { r: pix[0], g: pix[1], b: pix[2] } ).collect();
    let colors = split_into_buckets(&mut pixels, 1);
    colors
}

fn split_into_buckets(pixels: &mut [MyPixel], depth: u32) -> Vec<Color> {
    let mut result: Vec<Color> = Vec::with_capacity(16usize / depth as usize);
    if depth == 16 {
        let color = average_color(pixels);
        result.push(color);
    } else {
        let (r_range, g_range, b_range) = get_ranges(&pixels);
            if r_range >= g_range && r_range >= b_range {
                pixels.sort_by(|p1, p2| { MyPixel::compare_by(p1, p2, MyPixelOrderBy::R) });
            } else if g_range >= r_range && g_range >= b_range {
                pixels.sort_by(|p1, p2| { MyPixel::compare_by(p1, p2, MyPixelOrderBy::G) });
            } else {
                pixels.sort_by(|p1, p2| { MyPixel::compare_by(p1, p2, MyPixelOrderBy::B) });
            }

        let (mut bucket_l, mut bucket_r) = pixels.split_at_mut(pixels.len() / 2);
        result.extend(split_into_buckets(&mut bucket_l, depth * 2));
        result.extend(split_into_buckets(&mut bucket_r, depth * 2));
    }

    result
}

fn average_color(pixels: &[MyPixel]) -> Color {
    let mut r: u64 = 0;
    let mut g: u64 = 0;
    let mut b: u64 = 0;
    for pixel in pixels.iter() {
        r += pixel.r as u64;
        g += pixel.g as u64;
        b += pixel.b as u64;
    }
    r /= pixels.len() as u64;
    g /= pixels.len() as u64;
    b /= pixels.len() as u64;
    let r = r as u8;
    let g = g as u8;
    let b = b as u8;
    Color::new(r, g, b)
}
