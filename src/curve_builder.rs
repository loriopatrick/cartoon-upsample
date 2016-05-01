
use perimeter;
use shape::Shape;
use quadtree::QuadTree;
use models::{Point, Region};

pub fn get_points(shape: &Shape, width: usize, height: usize) -> Vec<Point> {
    let mut image = shape.rasterize(width, height);

    thin(&mut image, width, height);
    return perimeter::get_perimeter(image, width, height);
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
