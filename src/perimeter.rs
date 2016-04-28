
use quadtree::{QuadTree, Point, Region};
use shape_finder::Shape;

pub fn extract_perimeter(shape: &Shape, width: usize, height: usize) -> Vec<Point> {
    let img = rasterize(shape, width, height);
    return get_perimeter(img, width, height);
}

fn rasterize(shape: &Shape, width: usize, height: usize) -> Vec<bool> {
    let mut data = Vec::with_capacity((width + 2) * (height + 2));
    let w = width + 2;
    for part in &shape.parts {
        for y in part.region.y..part.region.y+part.region.height {
            for x in part.region.x..part.region.x+part.region.width {
                data[((x + 1) + (y + 1) * w as u32) as usize] = true;
            }
        }
    }
    return data;
}

const CIRCLE_X:[usize; 8] = [0, 1, 2, 2, 2, 1, 0, 0];
const CIRCLE_Y:[usize; 8] = [0, 0, 0, 1, 2, 2, 2, 1];

fn get_perimeter(image: Vec<bool>, width: usize, height: usize) -> Vec<Point> {
    let mut points = Vec::new();

    let mut cx = 0;
    let mut cy = 0;

    let w = width + 2;
    
    // Get a pixel cx, cy that is on the edge of our shape
    {
        let mut found = false;

        for y in 1..height {
            for x in 0..width {
                let pixel = image[x + y * w];
                if pixel {
                    found = true;
                    cy = y;
                    cx = x;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    let mut circle = [false; 8];
    let mut used = Vec::with_capacity(image.len() as usize);

    used[cx + cy * w] = true;
    points.push(Point{x: cx as u32, y: cy as u32});

    while true {
        for i in 0..8 {
            circle[i] = image[(cx + CIRCLE_X[i] - 1) + (cy + CIRCLE_Y[i] - 1) * w];
        }

        let mut empty_cidx = 0;
        for i in 0..8 {
            if !circle[i] {
                empty_cidx = i;
                break;
            }
        }

        let mut border_cidx = 0;
        let mut found = false;

        for i in empty_cidx..empty_cidx+8 {
            if circle[i % 8] && !used[(cx + CIRCLE_X[i] - 1) + (cy + CIRCLE_Y[i] - 1) * w] {
                border_cidx = i % 8;
                found = true;
                break;
            }
        }

        if !found {
            break;
        }

        cx += CIRCLE_X[border_cidx] - 1;
        cy += CIRCLE_Y[border_cidx] - 1;
        used[cx + cy * w] = true;
        points.push(Point{x: cx as u32, y: cy as u32});
    }

    return points;
}
