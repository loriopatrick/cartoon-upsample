
use quadtree::QuadTree;
use models::{Point, Region};
use shape::Shape;

pub fn extract_perimeter(shape: &Shape, width: usize, height: usize) -> Vec<Point> {
    let img = shape.rasterize(width, height);
    return get_perimeter(img, width, height);
}


const CIRCLE_X:[i64; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
const CIRCLE_Y:[i64; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
const CIRCLE_B:[i64; 8] = [7, 7, 1, 1, 3, 3, 5, 5];

pub fn get_perimeter(image: Vec<bool>, width: usize, height: usize) -> Vec<Point> {
    let mut points = Vec::new();

    let mut cx = 0i64;
    let mut cy = 0i64;

    let w = width as i64 + 2;
    
    // Get a pixel cx, cy that is on the edge of our shape
    {
        let mut found = false;

        for y in 1..height as i64 {
            for x in 0..width as i64 {
                let pixel = image[(x + w * y) as usize];
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
    let sx = cx;
    let sy = cy;
    points.push(Point{x: cx as u32, y: cy as u32});

    let mut last_empty_cid = 2 as usize;

    while true {
        for i in 0..8 {
            let idx = ((cx + CIRCLE_X[i]) + (cy + CIRCLE_Y[i]) * w);
            if idx < 0 {
                return points;
            }
            circle[i] = image[idx as usize];
        }
        
        let start_search = CIRCLE_B[last_empty_cid];

        for i in 1..8 {
            let idx = ((start_search + i) % 8) as usize;
            if circle[idx] {
                last_empty_cid = (idx + 7) % 8;
                break;
            }
        }

        // Move along border
        cx += CIRCLE_X[(last_empty_cid + 1) % 8];
        cy += CIRCLE_Y[(last_empty_cid + 1) % 8];

        if cx == sx && cy == sy {
            break;
        }

        points.push(Point{x: cx as u32, y: cy as u32});
    }

    return points;
}
