extern crate image;

use image::{RgbImage, Rgb};

fn is_kth_bit_set(n: u8, k:u8) -> bool {
    let temp: u8 =  1 << (k);
    n & temp != 0
}


fn rule_to_colors(rule:u8) -> Vec<Rgb<u8>> {
    let mut pix : Vec<Rgb<u8>> = vec![Rgb([255,255,255]); 8];
    let black = Rgb([0,0,0]);
    let colors: Vec<Rgb<u8>> = (0..8).into_iter().map(|n| {
        if is_kth_bit_set(rule, n)
        { Rgb([0,0,0])} 
        else {
            Rgb([255,255,255])
        }
    }).collect();
    
    colors
}

fn elem(generations: u32, rule: u8) -> image::RgbImage {

    let dimensions = 2 * generations + 1;
    let mut img = RgbImage::new(dimensions,dimensions);
    let black = Rgb([0,0,0]);
    let white = Rgb([255,255,255]);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255,255,255]);
    }
    img.put_pixel(generations, 0, Rgb([0,0,0]));


    let colors: Vec<Rgb<u8>> = rule_to_colors(rule);
    for y in 1..dimensions-1 {
        for x in 1..dimensions-1 {
            let up = img.get_pixel(x, y-1);
            let upleft = img.get_pixel(x-1, y-1);
            let upright = img.get_pixel(x+1, y-1);
            let up_b : bool = up[0] == black[0];
            let upr_b : bool = upright[0] == black[0];
            let upl_b : bool = upleft[0] == black[0];
            if (!upl_b && !up_b && !upr_b) {
                img.put_pixel(x, y, colors[0]);
            }
            if (!upl_b && !up_b && upr_b) {
                img.put_pixel(x, y, colors[1]);
            }
            if (!upl_b && up_b && !upr_b) {
                img.put_pixel(x, y, colors[2]);
            }
            if (!upl_b && up_b && upr_b) {
                img.put_pixel(x, y, colors[3]);
            }
            if (upl_b && !up_b && !upr_b) {
                img.put_pixel(x, y, colors[4]);
            }
            if (upl_b && !up_b && upr_b) {
                img.put_pixel(x, y, colors[5]);
            }
            if (upl_b && up_b && !upr_b) {
                img.put_pixel(x, y, colors[6]);
            }
            if (upl_b && up_b && upr_b) {
                img.put_pixel(x, y, colors[7]);
            }








        }
    }


    img

}
fn main() {
    println!("Running");
    let automata = elem(1000,30 );
    automata.save("test.png");   
    println!("Done!");   
}