extern crate image;

mod quadtree;
mod shape_finder;
mod debug_render;
mod perimeter;

use std::path::Path;
use image::{Pixel, Rgb};
use quadtree::QuadTree;

fn main() {
    let src = image::open(&Path::new("images/frame2.png")).unwrap().to_rgb();
    let im = image::imageops::resize(&src, 1024, 1024, image::FilterType::CatmullRom);

    let tree = QuadTree::build(&im, 10.0);

    let redraw = debug_render::render_quadtree(&tree);
    let lines = debug_render::render_quadtree_lines(&im, &tree);

    redraw.save(&Path::new("out/redraw.png")).unwrap();
    lines.save(&Path::new("out/lines.png")).unwrap();

    let mut option = Some(tree);

    let lines = shape_finder::take_shapes(&mut option, true);
    let shapes = shape_finder::take_shapes(&mut option, false);

    println!("lines {:?} shapes {:?}", lines.len(), shapes.len());

    let (width, height) = im.dimensions();
    let cool = debug_render::render_shapes(width, height, &shapes);
    cool.save(&Path::new("out/cool.png")).unwrap();

    for shape in &shapes {
        let points = perimeter::extract_perimeter(&shape, width as usize, height as usize);
        println!("Computed perimeter for shape, it has {:?} points", points.len());
    }
}
