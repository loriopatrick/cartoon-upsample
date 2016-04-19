extern crate image;

mod quadtree;
mod shape_finder;

use std::path::Path;

fn main() {
    let mut im = image::open(&Path::new("images/frame1.png")).unwrap().to_rgb();
    let _ = quadtree::build_tree(&mut im, 10.0);
    println!("I built a tree...");
}
