mod brightness;
mod color;
mod wal;
use brightness::Brightness;
use std::env;
use std::path::Path;
use wal::get;

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
    let palette = get(&picture, brightness);
    println!("{:#?}", palette);
}
