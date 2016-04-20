extern crate image;

mod quadtree;
mod shape_finder;
mod debug_render;

use std::path::Path;
use std::fs::File;
use quadtree::QuadTree;

fn main() {
    let mut im = image::open(&Path::new("images/frame1.png")).unwrap().to_rgb();
    let mut tree = QuadTree::build(&im, 10.0);

    let mut redraw = debug_render::render_quadtree(&tree);
    let mut lines = debug_render::render_quadtree_lines(&im, &tree);

    redraw.save(&Path::new("out/redraw.png")).unwrap();
    lines.save(&Path::new("out/lines.png")).unwrap();


    println!("region: {:?}", shape_finder::take_leaf(&mut tree.tl).unwrap().region);
    //println!("I built a tree... {:?}", leafs.unwrap().len());
}
