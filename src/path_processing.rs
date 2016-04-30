
use models::Point;

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
