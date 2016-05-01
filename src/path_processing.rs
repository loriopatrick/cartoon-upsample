
use models::{Point, Curve};

pub fn smooth(points: &mut Vec<Point>) {
    let kernel = [0.15, 0.25, 0.55, 0.75, 1.0, 0.7, 0.5, 0.2, 0.1];
    let offset = kernel.len() / 2;
    let kernel_sum = {
        let mut res = 0.0;
        for i in 0..kernel.len() {
            res += kernel[i];
        }
        res
    };

    for i in 0..points.len() {
        let mut px = 0.0;
        let mut py = 0.0;
        for k in 0..kernel.len() {
            let idx = ((i + points.len() + k - offset) % points.len()) as usize;
            px += kernel[k] * points[idx].x as f64;
            py += kernel[k] * points[idx].y as f64;
        }
        points[i].x = (px / kernel_sum) as u32;
        points[i].y = (py / kernel_sum) as u32;
    }
}

pub fn simplify(points: Vec<Point>, error_thres: f64) -> Vec<Point> {
    let mut res = Vec::new();

    let mut start_idx = 0;
    let end_idx = points.len() - 1;
    let end = points[end_idx];

    while start_idx < points.len() {
        let start = points[start_idx];

        res.push(start);

        let mut total_error = 0.0;
        let mut largest_error = 0.0;
        let mut largest_error_idx = 0;
        for i in start_idx+1..end_idx {
            let error = line_dist(start, end, points[i]).abs();
            if error > largest_error {
                largest_error = error;
                largest_error_idx = i;
            } else {
                largest_error *= 0.9;
            }

            total_error += error;
            if total_error > error_thres {
                start_idx = largest_error_idx;
                break;
            }
        }

        start_idx += 1;
    }

    return res;
}

fn line_dist(p1: Point, p2: Point, p: Point) -> f64 {
    let x1 = p1.x as f64;
    let y1 = p1.y as f64;
    let x2 = p2.x as f64;
    let y2 = p2.y as f64;
    let x0 = p.x as f64;
    let y0 = p.y as f64;

    return ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs() / ((y2 - y1).powf(2.0) + (x2 - x1).powf(2.0)).powf(0.5);
}
