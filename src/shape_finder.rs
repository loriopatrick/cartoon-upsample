extern crate image;

use image::Rgb;
use quadtree::{QuadTree, Point};

pub struct Shape {
    pub color: Rgb<u8>,
    pub parts: Vec<Box<QuadTree>>,
    pub area: f64,
}

pub fn take_shapes(option: &mut Option<Box<QuadTree>>, edges: bool) -> Vec<Shape> {
    let mut shapes = Vec::new();

    while true {
        let leaf = {
            let mut tree = option.as_mut().unwrap();
            let mut leaf = take_leaf(&mut tree.tl, edges);

            if leaf.is_none() {
                leaf = take_leaf(&mut tree.tr, edges);
                if leaf.is_none() {
                    leaf = take_leaf(&mut tree.bl, edges);
                    if leaf.is_none() {
                        leaf = take_leaf(&mut tree.br, edges);
                    }
                }
            }

            if leaf.is_none() {
                break;
            }

            leaf
        };

        let leaf_val = leaf.unwrap();
        shapes.push(find_shape(option, leaf_val, edges));
    }

    return shapes;
}

fn find_shape(tree: &mut Option<Box<QuadTree>>, start: Box<QuadTree>, do_edges: bool) -> Shape {
    let mut edges = Vec::new();
    let mut parts = Vec::new();

    let mut avg_color = start.color;
    let mut area = start.region.area();

    add_edges(&start, &mut edges);
    parts.push(start);

    while edges.len() > 0 {
        let (edge, src_color) = edges.pop().unwrap();

        let do_take = move |option: &Box<QuadTree>| {
            if do_edges {
                let area = option.region.area();
                let thres = 50;
                return area < 2.0;
            }
            let diff = color_diff(src_color, option.color);
            return diff < 30.0;
        };

        match take_by_edge(tree, &edge, &do_take) {
            Some(part) => {
                add_edges(&part, &mut edges);
                let part_area = part.region.area();
                let new_area = area + part_area;
                avg_color = Rgb([
                    ((avg_color[0] as f64 * area + part.color[0] as f64 * part_area) / new_area) as u8,
                    ((avg_color[1] as f64 * area + part.color[1] as f64 * part_area) / new_area) as u8,
                    ((avg_color[2] as f64 * area + part.color[2] as f64 * part_area) / new_area) as u8
                ]);
                area = new_area;
                parts.push(part);
            },
            None => continue
        }
    }

    return Shape{
        parts: parts,
        area: area,
        color: avg_color,
    };
}

fn add_edges(tree: &Box<QuadTree>, edges: &mut Vec<(Edge, Rgb<u8>)>) {
    let r = tree.region;
    let tl = Point{x:r.x, y:r.y};
    let tr = Point{x:r.x+r.width, y:r.y};
    let bl = Point{x:r.x, y:r.y+r.height};
    let br = Point{x:r.x+r.width, y:r.y+r.height};

    edges.push(((tl, tr), tree.color));
    edges.push(((tr, br), tree.color));
    edges.push(((br, bl), tree.color));
    edges.push(((bl, tl), tree.color));
}

type TakeFn = Fn(&Box<QuadTree>) -> bool;
type Edge = (Point, Point);

fn take_by_edge(cursor: &mut Option<Box<QuadTree>>, edge: &Edge, do_take: &TakeFn) -> Option<Box<QuadTree>> {
    if cursor.is_none() {
        return None;
    }

    {
        let mut tree = cursor.as_mut().unwrap();

        if !tree.is_leaf {
            if tree.region.contains(&edge.0) || tree.region.contains(&edge.1) {
                let mut res = take_by_edge(&mut tree.tl, edge, do_take);
                if res.is_some() { return res; }
                res = take_by_edge(&mut tree.tr, edge, do_take);
                if res.is_some() { return res; }
                res = take_by_edge(&mut tree.bl, edge, do_take);
                if res.is_some() { return res; }
                res = take_by_edge(&mut tree.br, edge, do_take);
                if res.is_some() { return res; }
            }
            return None;
        }

        if edge.0.x == edge.1.x {
            if tree.region.x != edge.0.x && tree.region.x + tree.region.width != edge.0.x {
                return None;
            }
            if (tree.region.y > edge.0.y || tree.region.y + tree.region.height < edge.0.y) &&
                    (tree.region.y > edge.1.y && tree.region.y + tree.region.height < edge.1.y) {
                return None;
            }
        } else {
            if tree.region.y != edge.0.y && tree.region.y + tree.region.width != edge.0.y {
                return None;
            }
            if (tree.region.x > edge.0.x || tree.region.x + tree.region.height < edge.0.x) &&
                    (tree.region.x > edge.1.x && tree.region.x + tree.region.height < edge.1.x) {
                return None;
            }
        }

        if !do_take(tree) {
            return None;
        }
    }

    return cursor.take();
}

pub fn take_leaf(cursor: &mut Option<Box<QuadTree>>, edges: bool) -> Option<Box<QuadTree>> {
    if cursor.is_none() {
        return None;
    }

    {
        let mut tree = cursor.as_mut().unwrap();

        if !tree.is_leaf {
            let mut res = take_leaf(&mut tree.tl, edges);
            if res.is_some() { return res; }
            res = take_leaf(&mut tree.tr, edges);
            if res.is_some() { return res; }
            res = take_leaf(&mut tree.bl, edges);
            if res.is_some() { return res; }
            res = take_leaf(&mut tree.br, edges);
            if res.is_some() { return res; }
            return None;
        }

        if edges && tree.region.area() > 2.0 {
            return None;
        }
    }

    return cursor.take();
}

fn pow2(a: f64) -> f64 {
    return a*a;
}

fn color_diff(a: Rgb<u8>, b: Rgb<u8>) -> f64 {
    // Source: http://www.compuphase.com/cmetric.htm
    let rmean = (a[0] as i64 + b[0] as i64) / 2;
    let r = a[0] as i64 - b[0] as i64;
    let g = a[1] as i64 - b[1] as i64;
    let b = a[2] as i64 - b[2] as i64;
    return f64::sqrt(((((512 + rmean) * r * r) >> 8) + 4 * g * g + (((767 - rmean) * b * b) >> 8)) as f64);
}
