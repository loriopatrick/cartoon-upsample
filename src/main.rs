extern crate image;

mod quadtree;
mod shape_finder;
mod neighbor;

use std::path::Path;

fn main() {
    let mut im = image::open(&Path::new("images/frame1.png")).unwrap().to_rgb();
    let mut tree = quadtree::build_tree(&mut im, 10.0);
    let item = neighbor::left(&mut tree);
    println!("I built a tree...");
}
