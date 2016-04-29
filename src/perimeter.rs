
use quadtree::{QuadTree, Point, Region};
use shape_finder::Shape;

pub fn extract_perimeter(shape: &Shape, width: usize, height: usize) -> Vec<Point> {
    let img = rasterize(shape, width, height);
    return get_perimeter(img, width, height);
}

fn rasterize(shape: &Shape, width: usize, height: usize) -> Vec<bool> {
    let len = (width + 2) * (height + 2);
    let mut data = Vec::with_capacity(len);

    for i in 0..len {
        data.push(false);
    }

    let w = width + 2;
    for part in &shape.parts {
        let sy = part.region.y as usize + 1;
        let ey = sy + part.region.height as usize;

        let sx = part.region.x as usize + 1;
        let ex = sx + part.region.width as usize;

        for y in sy..ey {
            for x in sx..ex {
                data[x + y * w] = true;
            }
        }
    }
    return data;
}

const CIRCLE_X:[i64; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
const CIRCLE_Y:[i64; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
const CIRCLE_B:[i64; 8] = [7, 7, 1, 1, 3, 3, 5, 5];

fn get_perimeter(image: Vec<bool>, width: usize, height: usize) -> Vec<Point> {
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
            circle[i] = image[((cx + CIRCLE_X[i]) + (cy + CIRCLE_Y[i]) * w) as usize];
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
