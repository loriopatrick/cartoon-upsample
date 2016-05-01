extern crate image;

mod models;
mod quadtree;
mod shape_finder;
mod debug_render;
mod perimeter;
mod path_processing;

use std::path::Path;
use image::{Pixel, Rgb};
use shape_finder::Shape;
use quadtree::QuadTree;

fn main() {
    let src = image::open(&Path::new("images/frame2.png")).unwrap().to_rgb();
    let im = image::imageops::resize(&src, 512, 512, image::FilterType::CatmullRom);

    let tree = QuadTree::build(&im, 10.0);

    let redraw = debug_render::render_quadtree(&tree);
    let lines = debug_render::render_quadtree_lines(&im, &tree);

    redraw.save(&Path::new("out/redraw.png")).unwrap();
    lines.save(&Path::new("out/lines.png")).unwrap();

    let mut option = Some(tree);

    let lines = shape_finder::take_shapes(&mut option, true);
    let shapes = shape_finder::take_shapes(&mut option, false);

    let (width, height) = im.dimensions();
    let cool = debug_render::render_shapes(width, height, &shapes);
    cool.save(&Path::new("out/cool.png")).unwrap();

    println!("<svg height=\"{}\" width=\"{}\">", width, height);
    println!("<g stroke-width=\"7\">");
    for shape in &shapes {
        if shape.area < 10.0 {
            continue;
        }
        let mut points = perimeter::extract_perimeter(&shape, width as usize, height as usize);
        path_processing::smooth(&mut points);
        points = path_processing::simplify(points, 5.0);
        print!("<path d=\"M{} {} ", points[0].x, points[0].y);
        let items = (points.len() - 1) / 3;
        for i in 0..items {
            let idx = i * 3;
            print!("C {} {}, {} {}, {} {}",
                   points[idx].x, points[idx].y,
                   points[idx + 1].x, points[idx + 1].y,
                   points[idx + 2].x, points[idx + 2].y
           );
        }
        println!("Z\" stroke=\"rgb({},{},{})\" fill=\"rgb({}, {}, {})\"/>", shape.color[0], shape.color[1], shape.color[2], shape.color[0], shape.color[1], shape.color[2]);
    }
    println!("</g>");
    println!("<g stroke-width=\"5\" fill=\"black\">");
    for shape in &lines {
        if shape.area < 10.0 {
            continue;
        }
        let mut points = perimeter::extract_perimeter(&shape, width as usize, height as usize);
        path_processing::smooth(&mut points);
        points = path_processing::simplify(points, 5.0);
        print!("<path d=\"M{} {} ", points[0].x, points[0].y);
        let items = (points.len() - 1) / 3;
        for i in 0..items {
            let idx = i * 3;
            print!("C {} {}, {} {}, {} {}",
                   points[idx].x, points[idx].y,
                   points[idx + 1].x, points[idx + 1].y,
                   points[idx + 2].x, points[idx + 2].y
           );
        }
        //println!("Z\" stroke=\"rgb({},{},{})\"/>", shape.color[0], shape.color[1], shape.color[2]);
        println!("Z\" />");
    }
    println!("</g>");
    println!("</svg>");
}
