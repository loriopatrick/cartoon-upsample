
use shape::Shape;
use quadtree::QuadTree;
use models::{Point, Region};

pub fn get_points(shape: &Shape) -> Vec<Vec<Point>> {
    let mut pre_points = Vec::new();

    let mut cx = 0;
    let mut cy = 0;
    let mut len = 0;
    for part in &shape.parts {
        let point = part.region.center();
        cx += point.x;
        cy += point.y;
        len += 1;
        if len == 5 {
            pre_points.push(Point{x: cx / 5, y: cy / 5});
            cx = 0;
            cy = 0;
            len = 0;
        }
    }

    let mut groups = Vec::new();

    let mut idx = 0;
    while idx < pre_points.len() {
        let mut points = Vec::new();
        let mut last_point = pre_points[idx];
        points.push(last_point);
        idx += 1;

        while idx < pre_points.len() {
            let point = pre_points[idx];
            if points[points.len() as usize - 1].distance(point) > 15.0 {
                break;
            }
            last_point = point;
            points.push(point);
            idx += 1;
        }

        if points.len() > 3 {
            groups.push(points);
        }
    }

    return groups;
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
        for y in 1..height+1 {
            for x in 1..w {
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

