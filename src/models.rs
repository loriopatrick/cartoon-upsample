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

impl Region {
    pub fn contains(&self, p: &Point) -> bool {
        return self.x <= p.x && self.y <= p.y &&
            self.x + self.width >= p.x && self.y + self.height >= p.y;
    }

    pub fn area(&self) -> f64 {
        return self.width as f64 * self.height as f64;
    }
}

