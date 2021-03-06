extern crate image;

use image::RgbImage;
use image::GenericImage;
use image::Rgb;

use models::{Region, Point};

pub type Pos = u8;

pub const TOP: Pos = 1;
pub const TL: Pos = 2;
pub const TR: Pos = 3;
pub const BL: Pos = 4;
pub const BR: Pos = 5;

pub struct QuadTree {
    pub region: Region,
    pub is_leaf: bool,
    pub color: Rgb<u8>,
    pub variance: f64,
    pub flag: u64,
    pub parent: *mut QuadTree,
    pub pos: Pos,
    pub tl: Option<Box<QuadTree>>,
    pub tr: Option<Box<QuadTree>>,
    pub bl: Option<Box<QuadTree>>,
    pub br: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn build(img: &RgbImage, thres: f64) -> Box<QuadTree> {
        let (width, height) = img.dimensions();
        let mut tree = tree_region(Region{
            x: 0,
            y: 0,
            width: width,
            height: height,
        }, TOP, 0 as *mut QuadTree);

        divide_tree(&mut tree, img, thres);
        return tree;
    }

    pub fn center(&self) -> Point {
        return Point{
            x: self.region.x + self.region.width / 2,
            y: self.region.y + self.region.height / 2,
        };
    }
}

fn divide_tree(tree: &mut QuadTree, img: &RgbImage, thres: f64) {
    let xi = tree.region.x;
    let xf = tree.region.x + tree.region.width;

    let yi = tree.region.y;
    let yf = tree.region.y+tree.region.height;

    let items = (xf - xi) as f64 * (yf - yi) as f64;

    let mut sum = Rgb{data: [0.0, 0.0, 0.0]};
    let mut var = 0 as f64;

    for x in xi..xf {
        for y in yi..yf {
            let pixel = img.get_pixel(x, y);
            sum[0] += pixel[0] as f64;
            sum[1] += pixel[1] as f64;
            sum[2] += pixel[2] as f64;
        }
    }

    let avg = Rgb{data: [
        (sum[0] / items) as u8,
        (sum[1] / items) as u8,
        (sum[2] / items) as u8,
    ]};

    for y in yi..yf {
        for x in xi..xf {
            let pixel = img.get_pixel(x, y);
            var += color_diff(&avg, pixel).powf(3.0);
        }
    }

    var /= items;

    tree.color = avg;
    tree.variance = var;
    tree.is_leaf == true;

    if var > thres && tree.region.width > 1 && tree.region.height > 1 {
        let w2 = tree.region.width / 2;
        let h2 = tree.region.height / 2;

        let parent = tree as *mut QuadTree;

        let mut tl = tree_region(Region{
            x: xi,
            y: yi,
            width: w2,
            height: h2,
        }, TL, parent);
        divide_tree(&mut tl, img, thres);
        tree.tl = Some(tl);

        let mut tr = tree_region(Region{
            x: xi + w2,
            y: yi,
            width: xf - (xi + w2),
            height: h2,
        }, TR, parent);
        divide_tree(&mut tr, img, thres);
        tree.tr = Some(tr);

        let mut bl = tree_region(Region{
            x: xi,
            y: yi + h2,
            width: w2,
            height: yf - (yi + h2),
        }, BL, parent);
        divide_tree(&mut bl, img, thres);
        tree.bl = Some(bl);

        let mut br = tree_region(Region{
            x: xi + w2,
            y: yi + h2,
            width: xf - (xi + w2),
            height: yf - (yi + h2),
        }, BR, parent);
        divide_tree(&mut br, img, thres);
        tree.br = Some(br);

        tree.is_leaf = false;
    }
}

fn pow2_diff<T:Into<f64>, U:Into<f64>>(a: T, b: U) -> f64 {
    let num = a.into() - b.into();
    return num * num;
}

fn tree_region(region: Region, pos: Pos, parent: *mut QuadTree) -> Box<QuadTree> {
    return Box::new(QuadTree{
        region: region,
        is_leaf: true,
        color: Rgb{ data: [0 as u8, 0 as u8, 0 as u8] },
        variance: 0.0,
        parent: parent,
        flag: 0,
        pos: pos,
        tl: None,
        tr: None,
        bl: None,
        br: None,
    });
}

fn color_diff(a: &Rgb<u8>, b: &Rgb<u8>) -> f64 {
    // Source: http://www.compuphase.com/cmetric.htm
    let rmean = (a[0] as i64 + b[0] as i64) / 2;
    let r = a[0] as i64 - b[0] as i64;
    let g = a[1] as i64 - b[1] as i64;
    let b = a[2] as i64 - b[2] as i64;
    return f64::sqrt(((((512 + rmean) * r * r) >> 8) + 4 * g * g + (((767 - rmean) * b * b) >> 8)) as f64);
}
