extern crate image;

use image::RgbImage;
use image::GenericImage;
use image::Rgb;

#[derive(Debug)]
pub struct Region {
    x: u32,
    y:u32,
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub struct QuadTree {
    region: Region,
    is_leaf: bool,
    tl: Option<Box<QuadTree>>,
    tr: Option<Box<QuadTree>>,
    bl: Option<Box<QuadTree>>,
    br: Option<Box<QuadTree>>,
}

pub fn build_tree(img: &mut RgbImage, thres: f64) -> Box<QuadTree> {
    let (width, height) = img.dimensions();
    let mut tree = tree_region(Region{
        x: 0,
        y: 0,
        width: width,
        height: height,
    });
    divide_tree(&mut tree, img, thres);
    return tree;
}

fn divide_tree(tree: &mut QuadTree, img: &mut RgbImage, thres: f64) {
    let xi = tree.region.x;
    let xf = tree.region.x+tree.region.width;

    let yi = tree.region.y;
    let yf = tree.region.y+tree.region.height;

    let items = (xf - xi) as f64 * (yf - yi) as f64;

    let mut sum = Rgb{data: [0.0, 0.0, 0.0]};
    let mut var = 0 as f64;

    for x in xi..xf {
        for y in yi..yf {
            let pixel = img.get_pixel(x, y);
            sum[0] += pixel[0] as f64;
            sum[1] += pixel[0] as f64;
            sum[2] += pixel[0] as f64;
        }
    }

    let avg = Rgb{data: [
        (sum[0] / items) as u8,
        (sum[1] / items) as u8,
        (sum[2] / items) as u8,
    ]};

    for x in xi..xf {
        for y in yi..yf {
            let pixel = img.get_pixel(x, y);
            var += pow2_diff(avg[0], pixel[0]);
            var += pow2_diff(avg[1], pixel[1]);
            var += pow2_diff(avg[2], pixel[2]);
        }
    }

    if var > thres {
        let w2 = tree.region.width / 2;
        let h2 = tree.region.height / 2;

        let mut tl = tree_region(Region{
            x: xi,
            y: yi,
            width: w2,
            height: h2,
        });
        divide_tree(&mut tl, img, thres);
        tree.tl = Some(tl);

        let mut tr = tree_region(Region{
            x: xi + w2,
            y: yi,
            width: w2,
            height: h2,
        });
        divide_tree(&mut tr, img, thres);
        tree.tr = Some(tr);

        let mut bl = tree_region(Region{
            x: xi,
            y: yi + h2,
            width: w2,
            height: h2,
        });
        divide_tree(&mut bl, img, thres);
        tree.bl = Some(bl);

        let mut br = tree_region(Region{
            x: xi + w2,
            y: yi + h2,
            width: w2,
            height: h2,
        });
        divide_tree(&mut br, img, thres);
        tree.br = Some(br);

        tree.is_leaf = false;
    }
}

fn pow2_diff<T:Into<f64>, U:Into<f64>>(a: T, b: U) -> f64 {
    let num = a.into() - b.into();
    return num * num;
}

fn tree_region(region: Region) -> Box<QuadTree> {
    return Box::new(QuadTree{
        region: region,
        is_leaf: true,
        tl: None,
        tr: None,
        bl: None,
        br: None,
    });
}
