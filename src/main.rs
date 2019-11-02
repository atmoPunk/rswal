extern crate image;

mod backend;
mod brightness;
mod color;
mod median_cut;
mod wal;
mod util;

use backend::get;
use brightness::Brightness;
use median_cut::MedianCutBackend;
use std::env;
use std::path::Path;
use wal::WalBackend;
use util::{set_bg, set_fg};

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

    let backend: Box<dyn crate::backend::Backend> = match args.get(3) {
        Some(b) => {
            if b == "wal" {
                Box::new(WalBackend {})
            } else {
                Box::new(MedianCutBackend {})
            }
        },
        None => {
            Box::new(MedianCutBackend {})
        }
    };
    let picture = Path::new(picture);
    let palette = get(&picture, brightness, backend);
    let bg = palette[0];
    for (i, color) in palette.into_iter().enumerate() {
        println!("{}Color {}: {}{} \x1b[37m\x1b[40m", set_bg(bg), i, set_fg(color), color);
    }
}
