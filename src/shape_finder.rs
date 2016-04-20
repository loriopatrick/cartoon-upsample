extern crate image;

use image::Rgb;
use quadtree::{QuadTree, Point};

pub fn take_shape(tree: &mut Box<QuadTree>) -> Option<Vec<Box<QuadTree>>> {
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
        return None;
    }

    let leaf_val = leaf.unwrap();

    let mut parts: Vec<Box<QuadTree>> = Vec::new();
    collect_parts(&mut tree.tl, &leaf_val, &mut parts);
    collect_parts(&mut tree.tr, &leaf_val, &mut parts);
    collect_parts(&mut tree.bl, &leaf_val, &mut parts);
    collect_parts(&mut tree.br, &leaf_val, &mut parts);
    parts.push(leaf_val);

    return Some(parts);
}

fn collect_parts(tree: &mut Option<Box<QuadTree>>, last_part: &Box<QuadTree>, parts: &mut Vec<Box<QuadTree>>) {
    if tree.is_none() {
        return
    }

    let (p1, p2, p3, p4) = {
        let r = &tree.as_ref().unwrap().region;
        (
            &Point{x:r.x, y:r.y},
            &Point{x:r.x, y:r.y+r.height},
            &Point{x:r.x+r.width, y:r.y},
            &Point{x:r.x+r.width, y:r.y+r.height},
        )
    };

    let color = last_part.color;
    let do_take = move |option: &Box<QuadTree>| {
        return color_diff(color, option.color) < 100.0;
    };

    collect_parts_by_edge(tree, &do_take, &(p1, p2), parts);
    collect_parts_by_edge(tree, &do_take, &(p2, p3), parts);
    collect_parts_by_edge(tree, &do_take, &(p3, p4), parts);
    collect_parts_by_edge(tree, &do_take, &(p3, p1), parts);
}

type TakeFn = Fn(&Box<QuadTree>) -> bool;
type Edge<'a> = (&'a Point, &'a Point);

fn collect_parts_by_edge(tree: &mut Option<Box<QuadTree>>, do_take: &TakeFn, edge: &Edge, parts: &mut Vec<Box<QuadTree>>) {
    match take_by_edge(tree, edge, do_take) {
        Some(x) => {
            collect_parts(tree, &x, parts);
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
            if (tree.region.y > edge.0.y || tree.region.y + tree.region.height < edge.0.y) &&
                    (tree.region.y > edge.1.y && tree.region.y + tree.region.height < edge.1.y) {
                return None;
            }
        } else {
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
