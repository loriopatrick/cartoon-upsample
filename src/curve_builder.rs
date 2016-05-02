
use perimeter;
use shape::Shape;
use quadtree::QuadTree;
use models::{Point, Region};
use imbin::ImBin;

pub fn get_points(shape: &Shape, im_buffer: &mut ImBin, clear_buffer: &mut ImBin) -> Vec<Vec<Point>> {
    shape.paint(im_buffer);
    thin(im_buffer, clear_buffer);

    let mut curves = Vec::new();
    clear_buffer.clear();

    println!("start");
    while true {
        println!("collect");
        let points = get_curve(im_buffer, clear_buffer);
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

fn get_curve(im_buffer: &mut ImBin, used_buffer: &mut ImBin) -> Vec<Point> {
    let width = im_buffer.width;
    let height = im_buffer.height;

    let mut cx = 0i64;
    let mut cy = 0i64;

    let w = width as i64 + 2;

    for y in 1..height as i64 {
        for x in 0..(width+1) as i64 {
            let idx = (x + y * w) as usize;
            if im_buffer.data[idx] && !used_buffer.data[idx] {
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

    used_buffer.data[(cx + cy * w) as usize] = true;
    points.push(Point{ x: cx as u32, y: cy as u32 });

    while true {
        let mut found = false;
        let mut pos = 0;
        for i in 0..8 {
            let idx = ((cx + CIRCLE_X[i]) + (cy + CIRCLE_Y[i]) * w) as usize;
            if im_buffer.data[idx] && !used_buffer.data[idx] {
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

        used_buffer.data[(cx + cy * w) as usize] = true;
        points.push(Point{ x: cx as u32, y: cy as u32 });
    }

    return points;
}


/*
 * Algorithm copied from
 * https://web.archive.org/web/20160314104646/http://opencv-code.com/quick-tips/implementation-of-guo-hall-thinning-algorithm/
 */
pub fn thin(im_buffer: &mut ImBin, clear_buffer: &mut ImBin) {
    let width = im_buffer.width;
    let height = im_buffer.height;

    let mut iter = 0;

    while iter < 4 {
        clear_buffer.clear();

        let w = width + 2;
        for y in 1..height {
            for x in 1..height {
                let p2 = bool_to_byte(im_buffer.data[x-1 + y * w]);
                let p3 = bool_to_byte(im_buffer.data[x-1 + (y+1) * w]);
                let p4 = bool_to_byte(im_buffer.data[x + (y+1) * w]);
                let p5 = bool_to_byte(im_buffer.data[x+1 + (y+1) * w]);
                let p6 = bool_to_byte(im_buffer.data[x+1 + y * w]);
                let p7 = bool_to_byte(im_buffer.data[x+1 + (y-1) * w]);
                let p8 = bool_to_byte(im_buffer.data[x + (y-1) * w]); 
                let p9 = bool_to_byte(im_buffer.data[x-1 + (y-1) * w]);



                let A = bool_to_byte(p2 == 0 && p3 == 1) + bool_to_byte(p3 == 0 && p4 == 1) + 
                    bool_to_byte(p4 == 0 && p5 == 1) + bool_to_byte(p5 == 0 && p6 == 1) + 
                    bool_to_byte(p6 == 0 && p7 == 1) + bool_to_byte(p7 == 0 && p8 == 1) +
                    bool_to_byte(p8 == 0 && p9 == 1) + bool_to_byte(p9 == 0 && p2 == 1);
                let B  = p2 + p3 + p4 + p5 + p6 + p7 + p8 + p9;
                let m1 = if iter == 0 { (p2 * p4 * p6) } else { (p2 * p4 * p8) };
                let m2 = if iter == 0 { (p4 * p6 * p8) } else { (p2 * p6 * p8) };

                if (A == 1 && (B >= 2 && B <= 6) && m1 == 0 && m2 == 0) {
                    clear_buffer.data[x + y * w] = true;
                }

                //let C = (!p2 & (p3 | p4)) + (!p4 & (p5 | p6)) +
                //        (!p6 & (p7 | p8)) + (!p8 & (p9 | p2));
                //let N1 = (p9 | p2) + (p3 | p4) + (p5 | p6) + (p7 | p8);
                //let N2 = (p2 | p3) + (p4 | p5) + (p6 | p7) + (p8 | p9);
                //let N = if N1 < N2 { N1 } else { N2 };
                //let m = if iter == 0 { ((p6 | p7 | !p9) & p8) } else { ((p2 | p3 | !p5) & p4) };

                //if C == 1 && bool_to_byte(N >= 2 && N <= 3) & m == 0 {
                //    clear_buffer.data[x + y * w] = true;
                //}
            }
        }

        let mut change = false;
        for i in 0..im_buffer.data.len() {
            if clear_buffer.data[i] && im_buffer.data[i] {
                im_buffer.data[i] = false;
                change = true;
            }
            clear_buffer.data[i] = false;
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

