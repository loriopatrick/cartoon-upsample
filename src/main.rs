extern crate image;

mod models;
mod quadtree;
mod shape;
mod debug_render;
mod perimeter;
mod path_processing;
mod curve_builder;

use std::path::Path;
use std::env;
use std::fs::File;
use std::io::Write;
use image::{Pixel, Rgb};
use shape::Shape;
use quadtree::QuadTree;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];

    println!("Processing {} to {}", input, output);

    let src = image::open(&Path::new(input)).unwrap().to_rgb();
    let im = image::imageops::resize(&src, 512, 512, image::FilterType::CatmullRom);
    println!("- preprocessed image");

    let tree = QuadTree::build(&im, 10.0);
    println!("- built tree");

    // Debug render
    // let redraw = debug_render::render_quadtree(&tree);
    // let lines = debug_render::render_quadtree_lines(&im, &tree);

    // redraw.save(&Path::new("out/redraw.png")).unwrap();
    // lines.save(&Path::new("out/lines.png")).unwrap();

    let mut option = Some(tree);

    let lines = Shape::take_shapes(&mut option, true);
    let shapes = Shape::take_shapes(&mut option, false);

    println!("- extracted features");

    let (width, height) = im.dimensions();

    //let cool = debug_render::render_shapes(width, height, &shapes);
    //cool.save(&Path::new("out/cool.png")).unwrap();

    let mut file = File::create(output).unwrap();

    write!(&mut file, "<svg height=\"{}\" width=\"{}\">\n", width, height).unwrap();
    write!(&mut file, "<g stroke-width=\"10\">\n").unwrap();
    for shape in &shapes {
        if shape.area < 10.0 {
            continue;
        }
        let mut points = perimeter::extract_perimeter(&shape, width as usize, height as usize);
        path_processing::smooth(&mut points);
        points = path_processing::simplify(points, 10.0);
        write!(&mut file, "<path d=\"M{} {} ", points[0].x, points[0].y).unwrap();
        let items = (points.len() - 1) / 3;
        for i in 0..items {
            let idx = i * 3;
            write!(&mut file, "C {} {}, {} {}, {} {}",
                   points[idx].x, points[idx].y,
                   points[idx + 1].x, points[idx + 1].y,
                   points[idx + 2].x, points[idx + 2].y
           ).unwrap();
        }
        write!(&mut file, "Z\" stroke=\"rgb({},{},{})\" fill=\"rgb({}, {}, {})\"/>\n", shape.color[0], shape.color[1], shape.color[2], shape.color[0], shape.color[1], shape.color[2]).unwrap();
    }
    write!(&mut file, "</g>\n").unwrap();
    write!(&mut file, "<g stroke-width=\"2\" stroke=\"black\" fill=\"none\">\n").unwrap();
    for shape in &lines {
        if shape.area < 10.0 {
            continue;
        }
        let mut groups = curve_builder::get_points(&shape);
        for mut points in groups {
            //path_processing::smooth(&mut points);
            //points = path_processing::simplify(points, 10.0);
            //path_processing::smooth(&mut points);
            write!(&mut file, "<path d=\"M{} {} ", points[0].x, points[0].y).unwrap();
            let items = (points.len() - 1) / 3;
            for i in 0..items {
                let idx = i * 3;
                write!(&mut file, "C {} {}, {} {}, {} {}",
                       points[idx].x, points[idx].y,
                       points[idx + 1].x, points[idx + 1].y,
                       points[idx + 2].x, points[idx + 2].y
               ).unwrap();
            }
            write!(&mut file, "\" />\n").unwrap();
        }
    }
    write!(&mut file, "</g>").unwrap();
    write!(&mut file, "</svg>").unwrap();
}
