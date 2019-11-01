extern crate image;

mod backend;
mod brightness;
mod color;
mod median_cut;
mod wal;

use backend::get;
use brightness::Brightness;
use median_cut::MedianCutBackend;
use std::env;
use std::path::Path;
use wal::WalBackend;

fn main() {
    let args: Vec<String> = env::args().collect();
    let picture = args.get(1).expect("Please enter a path to a picture");
    let brightness = match args.get(2) {
        Some(b) => {
            if b == "-l" {
                Brightness::Light
            } else if b == "-d" {
                Brightness::Dark
            } else {
                panic!("Unexpected argument: {:?}", b)
            }
        }
        None => Brightness::Dark,
    };
    let picture = Path::new(picture);
    let wbe = WalBackend {};
    let mcbe = MedianCutBackend {};
    let palette = get(&picture, brightness, wbe);
    for (i, color) in palette.iter().enumerate() {
        println!("Color {}: {}", i, color);
    }
    println!("=====");
    let palette = get(&picture, brightness, mcbe);
    for (i, color) in palette.iter().enumerate() {
        println!("Color {}: {}", i, color);
    }
}
