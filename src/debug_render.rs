extern crate rand;
extern crate image;

use self::rand::Rng;
use models::Point;
use image::{
    Rgb,
    RgbImage,
    ImageBuffer,
    Pixel
};
use quadtree::QuadTree;
use shape_finder::Shape;

pub fn render_quadtree_lines(base: &RgbImage, tree: &Box<QuadTree>) -> RgbImage {
    return ImageBuffer::from_fn(tree.region.width, tree.region.height, |x, y| {
        let p = Point{x: x, y: y};
        if on_edge(&p, &tree) {
            Rgb([0 as u8, 0 as u8, 0 as u8])
        } else {
            base.get_pixel(x, y).to_rgb()
        }
    });
}

pub fn render_quadtree(tree: &Box<QuadTree>) -> RgbImage {
    return ImageBuffer::from_fn(tree.region.width, tree.region.height, |x, y| {
        match get_tree(&Point{x: x, y: y}, tree) {
            None => Rgb([0 as u8; 3]),
            Some(x) => x.color,
        }
    });
}

pub fn render_shapes(width: u32, height: u32, shapes: &Vec<Shape>) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();
    for shape in shapes {
        let mut _color = [0 as u8; 3];
        rng.fill_bytes(&mut _color);
        let mut color = Rgb(_color);

        for item in &shape.parts {
            for x in item.region.x..item.region.x+item.region.width {
                for y in item.region.y..item.region.y+item.region.height {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
    return img;
}

fn get_tree<'a>(p: &Point, root: &'a Box<QuadTree>) -> Option<&'a Box<QuadTree>> {
    let mut res = get_tree_child(p, &root.tl);
    if res.is_some() { return res; }
    res = get_tree_child(p, &root.tr);
    if res.is_some() { return res; }
    res = get_tree_child(p, &root.bl);
    if res.is_some() { return res; }
    res = get_tree_child(p, &root.br);
    return res;
}

fn get_tree_child<'a>(p: &Point, _tree: &'a Option<Box<QuadTree>>) -> Option<&'a Box<QuadTree>> {
    if _tree.is_none() { return None; }
    let tree = _tree.as_ref().unwrap();

    if !tree.region.contains(p) {
        return None;
    }

    if tree.is_leaf {
        return Some(tree);
    }

    let mut res = get_tree_child(p, &tree.tl);
    if res.is_some() { return res; }
    res = get_tree_child(p, &tree.tr);
    if res.is_some() { return res; }
    res = get_tree_child(p, &tree.bl);
    if res.is_some() { return res; }
    res = get_tree_child(p, &tree.br);
    return res;
}

fn on_edge(p: &Point, tree: &Box<QuadTree>) -> bool {
    return on_edge_child(p, &tree.tl) || on_edge_child(p, &tree.tr)
        || on_edge_child(p, &tree.bl) || on_edge_child(p, &tree.br);
}

fn on_edge_child(p: &Point, _tree: &Option<Box<QuadTree>>) -> bool {
    if _tree.is_none() { return false; }
    let tree = _tree.as_ref().unwrap();

    if !tree.region.contains(p) {
        return false;
    }

    let r = &tree.region;

    if r.x == p.x || r.x + r.width == p.x ||
            r.y == p.y || r.y + r.height == p.y {
        return true;
    }

    return on_edge_child(p, &tree.tl) || on_edge_child(p, &tree.tr)
        || on_edge_child(p, &tree.bl) || on_edge_child(p, &tree.br);
}
