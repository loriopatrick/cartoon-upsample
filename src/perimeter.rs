
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
    let mut used = Vec::with_capacity(image.len() as usize);
    for i in 0..image.len() {
        used.push(false);
    }

    used[(cx + cy * w) as usize] = true;
    points.push(Point{x: cx as u32, y: cy as u32});

    let mut backtrace = 0i64;

    while true {
        for i in 0..8 {
            circle[i] = image[((cx + CIRCLE_X[i]) + (cy + CIRCLE_Y[i]) * w) as usize];
        }

        // state = 0, no pixel
        // state = 1, used pixel
        // state = 2, pixel

        let mut last_state = 0;
        if used[((cx + CIRCLE_X[7]) + (cy + CIRCLE_Y[7]) * w) as usize] {
            last_state = 1;
        } else if circle[7] {
            last_state = 2;
        }

        let mut found = false;
        let mut next_cidx = 10 as i64;
        for i in 0..8 {
            let state = {
                if used[((cx + CIRCLE_X[i]) + (cy + CIRCLE_Y[i]) * w) as usize] {
                    1
                } else if circle[i] {
                    2
                } else {
                    0
                }
            };

            if last_state == 0 && state == 2 {
                next_cidx = i as i64;
                break;
            } else if last_state == 2 && state == 0 {
                next_cidx = i as i64 - 1;
                break;
            }

            last_state = state;
        }

        if next_cidx == 10 {
            backtrace += 1;
            let len = points.len() as i64;
            if len == backtrace || backtrace > 10 {
                break;
            }
            cx = points[(len - backtrace) as usize].x as i64;
            cy = points[(len - backtrace) as usize].y as i64;
            continue;
        }

        backtrace = 0;

        next_cidx = (next_cidx + 8) % 8;

        // Move along border
        cx += CIRCLE_X[next_cidx as usize];
        cy += CIRCLE_Y[next_cidx as usize];

        used[(cx + cy * w) as usize] = true;
        points.push(Point{x: cx as u32, y: cy as u32});
    }

    return points;
}
