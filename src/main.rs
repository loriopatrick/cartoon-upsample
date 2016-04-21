extern crate image;

mod quadtree;
mod shape_finder;
mod debug_render;

use std::path::Path;
use image::{Pixel, Rgb};
use quadtree::QuadTree;

fn main() {
    let src = image::open(&Path::new("images/frame2.png")).unwrap().to_rgb();
    let im = image::imageops::resize(&src, 512, 512, image::FilterType::Nearest);


    let tree = QuadTree::build(&im, 7.0);

    let redraw = debug_render::render_quadtree(&tree);
    let lines = debug_render::render_quadtree_lines(&im, &tree);

    redraw.save(&Path::new("out/redraw.png")).unwrap();
    lines.save(&Path::new("out/lines.png")).unwrap();

    let mut shapes = shape_finder::take_shapes(tree);
    shapes.sort_by_key(|item| {
        return item.area as i64;
    });

    let (width, height) = im.dimensions();
    let cool = debug_render::render_shapes(width, height, &shapes);
    cool.save(&Path::new("out/cool.png")).unwrap();
}
