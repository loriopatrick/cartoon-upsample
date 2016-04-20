extern crate image;

mod quadtree;
mod shape_finder;

use std::path::Path;
use quadtree::QuadTree;

fn main() {
    let mut im = image::open(&Path::new("images/frame1.png")).unwrap().to_rgb();
    let (width, height) = im.dimensions();
    let mut tree = QuadTree::build(&mut im, 10.0);
    let leafs = shape_finder::take_shape(&mut tree);
    println!("I built a tree... {:?}", leafs.unwrap().len());
}
