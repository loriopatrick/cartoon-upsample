
use perimeter;
use shape::Shape;
use quadtree::QuadTree;
use models::{Point, Region};

pub fn get_points(shape: &Shape, width: usize, height: usize) -> Vec<Vec<Point>> {
    let mut image = shape.rasterize(width, height);
    thin(&mut image, width, height);

    let mut curves = Vec::new();
    let mut used = Vec::with_capacity(image.len());
    used.resize(image.len(), false);

    println!("start");
    while true {
        println!("collect");
        let points = get_curve(&image, &mut used, width, height);
        if points.len() == 0 {
            break;
        }
        curves.push(points);
    }

    println!("done");
    return curves;
}

const CIRCLE_X:[i64; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
const CIRCLE_Y:[i64; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];

fn get_curve(image: &Vec<bool>, used: &mut Vec<bool>, width: usize, height: usize) -> Vec<Point> {
    let mut cx = 0i64;
    let mut cy = 0i64;

    let w = width as i64 + 2;

    for y in 1..height as i64 {
        for x in 0..(width+1) as i64 {
            let idx = (x + y * w) as usize;
            if image[idx] && !used[idx] {
                cx = x as i64;
                cy = y as i64;
                break;
            }
        }
    }

    let mut points = Vec::new();

    if cx == 0 && cy == 0 {
        return points;
    }

    used[(cx + cy * w) as usize] = true;
    points.push(Point{ x: cx as u32, y: cy as u32 });

    while true {
        let mut found = false;
        let mut pos = 0;
        for i in 0..8 {
            let idx = ((cx + CIRCLE_X[i]) + (cy + CIRCLE_Y[i]) * w) as usize;
            if image[idx] && !used[idx] {
                pos = i;
                found = true;
                break;
            }
        }

        if !found {
            break;
        }

        cx += CIRCLE_X[pos];
        cy += CIRCLE_Y[pos];

        used[(cx + cy * w) as usize] = true;
        points.push(Point{ x: cx as u32, y: cy as u32 });
    }

    return points;
}


/*
 * Algorithm copied from
 * https://web.archive.org/web/20160314104646/http://opencv-code.com/quick-tips/implementation-of-guo-hall-thinning-algorithm/
 */
fn thin(image: &mut Vec<bool>, width: usize, height: usize) {
    let mut to_clear = Vec::with_capacity(image.len());
    to_clear.resize(image.len(), false);

    let mut iter = 0;

    while iter < 10 {
        let w = width + 2;
        for y in 1..height {
            for x in 1..height {
                let p2 = bool_to_byte(image[x-1 + y * w]);
                let p3 = bool_to_byte(image[x-1 + (y+1) * w]);
                let p4 = bool_to_byte(image[x + (y+1) * w]);
                let p5 = bool_to_byte(image[x+1 + (y+1) * w]);
                let p6 = bool_to_byte(image[x+1 + y * w]);
                let p7 = bool_to_byte(image[x+1 + (y-1) * w]);
                let p8 = bool_to_byte(image[x + (y-1) * w]); 
                let p9 = bool_to_byte(image[x-1 + (y-1) * w]);

                let C = (!p2 & (p3 | p4)) + (!p4 & (p5 | p6)) +
                        (!p6 & (p7 | p8)) + (!p8 & (p9 | p2));
                let N1 = (p9 | p2) + (p3 | p4) + (p5 | p6) + (p7 | p8);
                let N2 = (p2 | p3) + (p4 | p5) + (p6 | p7) + (p8 | p9);
                let N = if N1 < N2 { N1 } else { N2 };
                let m = if iter == 0 { ((p6 | p7 | !p9) & p8) } else { ((p2 | p3 | !p5) & p4) };

                if C == 1 && bool_to_byte(N >= 2 && N <= 3) & m == 0 {
                    to_clear[x + y * w] = true;
                }
            }
        }

        let mut change = false;
        for i in 0..image.len() {
            if to_clear[i] && image[i] {
                image[i] = false;
                change = true;
            }
            to_clear[i] = false;
        }

        if !change {
            break;
        }

        iter += 1;
    }
}

fn bool_to_byte(b: bool) -> u8 {
    return {
        if b {
            1
        } else {
            0
        }
    };
}

