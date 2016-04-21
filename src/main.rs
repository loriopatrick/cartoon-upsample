extern crate image;

mod quadtree;
mod shape_finder;
mod debug_render;

use std::path::Path;
use quadtree::QuadTree;

fn main() {
    let src = image::open(&Path::new("images/southpark5.png")).unwrap().to_rgb();
    let im = image::imageops::resize(&src, 1024, 1024, image::FilterType::Nearest);

    let tree = QuadTree::build(&im, 5.0);

    let redraw = debug_render::render_quadtree(&tree);
    let lines = debug_render::render_quadtree_lines(&im, &tree);

    redraw.save(&Path::new("out/redraw.png")).unwrap();
    lines.save(&Path::new("out/lines.png")).unwrap();

    let shapes = shape_finder::take_shapes(tree);

    let (width, height) = im.dimensions();
    let cool = debug_render::render_shapes(width, height, &shapes);
    cool.save(&Path::new("out/cool.png")).unwrap();
}
