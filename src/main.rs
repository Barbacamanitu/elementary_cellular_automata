extern crate image;
use std::time::{Duration, Instant};
use image::{RgbImage, Rgb};


struct Config {
    rule: u8,
    generations: u32,
    alg: u8
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let rule: u8 = match args.next() {
            Some(arg) => {
                let parse_res = arg.parse::<u8>();
                match parse_res {
                    Err(err) => return Err("Couldnt parse rule"),
                    Ok(num) => num
                }
            },
            None => return Err("Didn't get a rule"),
        };

        let generations: u32 = match args.next() {
            Some(arg) => {
                let parse_res = arg.parse::<u32>();
                match parse_res {
                    Err(err) => return Err("Couldnt parse generations"),
                    Ok(num) => num
                }
            },
            None => return Err("Didn't get a generation count"),
        };


        let alg: u8 = match args.next() {
            Some(arg) => { 
                match arg.parse::<u8>() {
                    Ok(arg_num) => { arg_num}
                    Err(_) => { 0u8 }
                }
            }
            None => { 0u8}
        };

        Ok(Config{
            rule: rule,
            generations: generations,
            alg: alg
        })
    }
}


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

fn elem(generations: u32, rule: u8) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {

    let w: u32 = generations * 2 + 1;
    let h: u32 = generations;
    let mut img = RgbImage::new(w,h);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255,255,255]);
    }
    img.put_pixel(generations, 0, Rgb([0,0,0]));


    let colors: Vec<Rgb<u8>> = rule_to_colors(rule);
    for y in 1..h-1 {
        for x in 1..w-1 {
            let up = img.get_pixel(x, y-1);
            let upleft = img.get_pixel(x-1, y-1);
            let upright = img.get_pixel(x+1, y-1);
            let up_b : bool = up[0] == 0;
            let upr_b : bool = upright[0] == 0;
            let upl_b : bool = upleft[0] == 0;
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


fn xy_to_idx(x: u32, y: u32, width: u32) -> usize {
    (y * width + x) as usize
}



fn elem_opt1(generations: u32, rule: u8) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let w: u32 = generations * 2 + 1;
    let h: u32 = generations;
    let mut img = RgbImage::new(w,h);
    let mut cells: Vec<u8> = vec![0u8; (w*h) as usize];

    cells[xy_to_idx(generations, 0, w)] = 1;

    for y in 1..h {
        for x in 1..(w-1) {
            let up_left = cells[xy_to_idx(x-1, y-1, w)];
            let up = cells[xy_to_idx(x, y-1, w)];
            let up_right = cells[xy_to_idx(x+1, y-1, w)];

            let rule_idx = up_right + (up * 2) + (up_left * 4);
            if (is_kth_bit_set(rule, rule_idx)) {
                cells[xy_to_idx(x, y, w)] = 1;
            }
        }
    }

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let idx = xy_to_idx(x, y, w);
        let cell = cells[idx];
        let col = 255*(1-cell);
        *pixel = Rgb([col,col,col]);
    }


    img
}



struct AutomataRunner
{
    rule: u8,
    generations: u32,
    algorithm: fn(u32,u8) -> image::RgbImage,
    opt_level: i32
}

impl AutomataRunner 
{
    pub fn new(config: Config) -> Result<AutomataRunner,&'static str> {
        let rule = config.rule;
        let gens = config.generations;
        let opt0 = elem as fn(u32,u8) -> image::RgbImage;
        let opt1 = elem_opt1 as fn(u32,u8) -> image::RgbImage;
        let (alg,level) = match config.alg {
            0 => (opt0,0),
            1 =>  (opt1,1),
            _ => return Err("Could not find algorithm.")
        };

        Ok(AutomataRunner {
            rule: rule,
            generations: gens,
            algorithm: alg,
            opt_level: level
        })
    }


    pub fn run(&self) {
        let timer0 = Instant::now();
        println!("Running algorithm Optimation:{} for {} generations...",self.opt_level,self.generations);
        let automata_buf = (self.algorithm)(self.generations,self.rule);
        let elapsed_secs = timer0.elapsed().as_secs_f32();
        println!("Completed in {} seconds.",elapsed_secs);
        let save_timer0 = Instant::now();
        let fname = format!("rule-{},gens-{},opt-{}.png",self.rule,self.generations,self.opt_level);
        automata_buf.save(fname.clone()).unwrap();    
        let saved_secs = save_timer0.elapsed().as_secs_f32();
        println!("Saved file {} in {} seconds",fname,saved_secs);
    }
}



fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

   let runner = AutomataRunner::new(config);
   match runner {
       Ok(r) => { r.run(); }
       Err(_) => { eprintln!("Couldn't create automata runner.")}
   };

    
}