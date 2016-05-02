extern crate image;

mod models;
mod quadtree;
mod shape;
mod debug_render;
mod perimeter;
mod path_processing;
mod curve_builder;
mod imbin;

use std::path::Path;
use std::env;
use std::fs::File;
use std::io::Write;
use image::{Pixel, Rgb};
use shape::Shape;
use quadtree::QuadTree;
use imbin::ImBin;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];

    println!("Processing {} to {}", input, output);

    let src = image::open(&Path::new(input)).unwrap().to_rgb();
    let im = image::imageops::resize(&src, 2048, 2048, image::FilterType::CatmullRom);
    println!("- preprocessed image");

    let tree = QuadTree::build(&im, 200.0);
    println!("- built tree");

    //let redraw = debug_render::render_quadtree(&tree);
    //let lines = debug_render::render_quadtree_lines(&im, &tree);
    //redraw.save(&Path::new("out/redraw.png")).unwrap();
    //lines.save(&Path::new("out/lines.png")).unwrap();
    //return;

    let mut option = Some(tree);

    let lines = Shape::take_shapes(&mut option, true);
    let shapes = Shape::take_shapes(&mut option, false);

    println!("- extracted features");

    let (width, height) = im.dimensions();
    let mut im_buf1 = ImBin::new(width as usize, height as usize);
    let mut im_buf2 = ImBin::new(width as usize, height as usize);

    //im_buf1.clear();
    //for line in &lines {
    //    line.paint(&mut im_buf1);
    //}
    //curve_builder::thin(&mut im_buf1, &mut im_buf2);
    //debug_render::render_imbin(&im_buf1).save(&Path::new("out/test.png")).unwrap();
    //return;


    let mut file = File::create(output).unwrap();

    write!(&mut file, "<svg height=\"{}\" width=\"{}\">\n", width, height).unwrap();
    write!(&mut file, "<g stroke-width=\"10\">\n").unwrap();
    for shape in &shapes {
        if shape.area < 100.0 {
            continue;
        }
        im_buf1.clear();
        let mut points = perimeter::extract_perimeter(&shape, &mut im_buf1);
        path_processing::smooth(&mut points);
        points = path_processing::simplify(points, 10.0);
        path_processing::smooth(&mut points);
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
    //write!(&mut file, "<g stroke-width=\"5\" stroke=\"black\" fill=\"none\">\n").unwrap();
    //for shape in &lines {
    //    if shape.area < 100.0 {
    //        continue;
    //    }
    //    im_buf1.clear();
    //    im_buf2.clear();
    //    let mut curves = curve_builder::get_points(&shape, &mut im_buf1, &mut im_buf2);
    //    for mut points in curves {
    //        points = path_processing::simplify(points, 10.0);
    //        write!(&mut file, "<path d=\"M{} {} ", points[0].x, points[0].y).unwrap();
    //        let items = (points.len() - 1) / 3;
    //        for i in 0..items {
    //            let idx = i * 3;
    //            write!(&mut file, "C {} {}, {} {}, {} {}",
    //                   points[idx].x, points[idx].y,
    //                   points[idx + 1].x, points[idx + 1].y,
    //                   points[idx + 2].x, points[idx + 2].y
    //           ).unwrap();
    //        }
    //        write!(&mut file, "\" />\n").unwrap();
    //    }
    //}
    //write!(&mut file, "</g>").unwrap();
    write!(&mut file, "</svg>").unwrap();
    file.flush();
}
