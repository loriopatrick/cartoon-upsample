extern crate image;

use std::path::Path;
use image::GenericImage;

fn main() {
    let im = image::open(&Path::new("images/frame1.png")).unwrap();
    let (width, height) = im.dimensions();
    println!("size: {:?}", (width, height));
    println!("color: {:?}", im.color());

    let mut pixels = im.pixels();
    println!("value at (3,3) {:?}", pixels.nth((3 * width + 3) as usize));
}
