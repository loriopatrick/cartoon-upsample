#[derive(Debug, Copy, Clone)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Curve {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Region {
    pub fn contains(&self, p: &Point) -> bool {
        return self.x <= p.x && self.y <= p.y &&
            self.x + self.width >= p.x && self.y + self.height >= p.y;
    }

    pub fn area(&self) -> f64 {
        return self.width as f64 * self.height as f64;
    }
}

impl Point {
    pub fn distance(self, p: Point) -> f64 {
        return ((self.x as f64 - p.x as f64).powf(2.0) + (self.y as f64 - p.y as f64).powf(2.0)).powf(0.5);
    }

    pub fn add(self, p: Point) -> Point {
        return Point{
            x: self.x + p.x,
            y: self.y + p.y,
        };
    }

    pub fn normalize(self, dist: f64) -> Point {
        let scale = dist / ((self.x as f64).powf(2.0) + (self.y as f64).powf(2.0)).powf(0.5);
        return Point{
            x: (self.x as f64 * scale) as u32,
            y: (self.y as f64 * scale) as u32,
        };
    }
}
