
pub struct ImBin {
    pub data: Vec<bool>,
    pub width: usize,
    pub height: usize,
    pub wrow: usize,
}

impl ImBin {
    pub fn new(width: usize, height: usize) -> ImBin {
        let len = (width + 2) * (height + 2);
        let mut data = Vec::with_capacity(len);
        data.resize(len, false);
        return ImBin{ data: data, width: width, height: height, wrow: width + 2 };
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = false;
        }
    }
}
