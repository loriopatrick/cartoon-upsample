extern crate image;

use image::Rgb;
use quadtree::{QuadTree, Point};

pub fn take_shapes(tree: Box<QuadTree>) -> Vec<Vec<Box<QuadTree>>> {
    let mut tree_root = Some(tree);

    let mut shapes = Vec::new();
    while true {
        let leaf = {
            let tree = tree_root.as_mut().unwrap();
            let mut leaf = take_leaf(&mut tree.tl);

            if leaf.is_none() {
                leaf = take_leaf(&mut tree.tr);
                if leaf.is_none() {
                    leaf = take_leaf(&mut tree.bl);
                    if leaf.is_none() {
                        leaf = take_leaf(&mut tree.br);
                    }
                }
            }

            if leaf.is_none() {
                return break;
            }

            leaf
        };

        let mut parts: Vec<Box<QuadTree>> = Vec::new();
        let leaf_val = leaf.unwrap();
        collect_parts(&mut tree_root, &leaf_val, &leaf_val, &mut parts);
        parts.push(leaf_val);
        shapes.push(parts);
    }

    return shapes;
}

fn collect_parts(tree: &mut Option<Box<QuadTree>>, first_part: &Box<QuadTree>, last_part: &Box<QuadTree>, parts: &mut Vec<Box<QuadTree>>) {
    if tree.is_none() {
        return
    }

    let (tl, tr, bl, br) = {
        let r = last_part.region;
        (
            Point{x:r.x, y:r.y},
            Point{x:r.x+r.width, y:r.y},
            Point{x:r.x, y:r.y+r.height},
            Point{x:r.x+r.width, y:r.y+r.height},
        )
    };

    let color = first_part.color;
    let region = last_part.region;
    let last_color = last_part.color;
    let do_take = move |option: &Box<QuadTree>| {
        return color_diff(color, option.color) < 100.0 ||
            region.area() + option.region.area() < 10.0 && color_diff(last_color, option.color) < 100.0;
    };

    collect_parts_by_edge(tree, first_part, &do_take, &(tl, tr), parts);
    collect_parts_by_edge(tree, first_part, &do_take, &(tr, br), parts);
    collect_parts_by_edge(tree, first_part, &do_take, &(br, bl), parts);
    collect_parts_by_edge(tree, first_part, &do_take, &(bl, tl), parts);
}

type TakeFn = Fn(&Box<QuadTree>) -> bool;
type Edge = (Point, Point);

fn collect_parts_by_edge(tree: &mut Option<Box<QuadTree>>, first_part: &Box<QuadTree>, do_take: &TakeFn, edge: &Edge, parts: &mut Vec<Box<QuadTree>>) {
    match take_by_edge(tree, edge, do_take) {
        Some(x) => {
            collect_parts(tree, first_part, &x, parts);
            parts.push(x);
        },
        _ => return,
    }
}

pub fn take_leaf(cursor: &mut Option<Box<QuadTree>>) -> Option<Box<QuadTree>> {
    if cursor.is_none() {
        return None;
    }

    {
        let mut tree = cursor.as_mut().unwrap();

        if !tree.is_leaf {
            let mut res = take_leaf(&mut tree.tl);
            if res.is_some() { return res; }
            res = take_leaf(&mut tree.tr);
            if res.is_some() { return res; }
            res = take_leaf(&mut tree.bl);
            if res.is_some() { return res; }
            res = take_leaf(&mut tree.br);
            if res.is_some() { return res; }
            return None;
        }
    }

    return cursor.take();
}

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

fn color_diff(a: Rgb<u8>, b: Rgb<u8>) -> f64 {
    return diff_pow2_u8(a[0], b[0]) + diff_pow2_u8(a[1], b[1]) + diff_pow2_u8(a[2], b[2]);
}

fn diff_pow2_u8(a: u8, b: u8) -> f64 {
    let x = a as f64 - b as f64;
    return x * x;
}
