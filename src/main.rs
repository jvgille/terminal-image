extern crate termion;
extern crate image;

use image::*;
use termion::color;

fn set_bg(r : u8, g : u8, b : u8) {
    print!("{}", color::Bg(color::Rgb(r, g, b)));
}

fn set_fg(r : u8, g : u8, b : u8) {
    print!("{}", color::Fg(color::Rgb(r, g, b)));
}

// calculates the downscaling factor necessary to fit the image inside the terminal
fn calc_downscale(image: &DynamicImage) -> u32 {
    let (t_width, t_height) = termion::terminal_size().unwrap();
    let h_scale = (image.width() as f64/t_width as f64).ceil();
    let v_scale = (image.height() as f64/(t_height*2) as f64).ceil();
    if h_scale > v_scale {
        h_scale as u32
    } else {
        v_scale as u32
    }
}

// gets pixel at x, y coordinate from rgba image buffer
fn get_pixel(buf: &Vec<u8>, image: &DynamicImage, x: u32, y: u32) -> (u8, u8, u8, u8) {
    let base = ((y*image.width() + x) * 4) as usize;
    (buf[base], buf[base+1], buf[base+2], buf[base+3])
}

// samples a square of size² elements and returns the average pixel, taking alpha into account
fn sample(buf: &Vec<u8>, image: &DynamicImage, x: u32, y: u32, size: u32) -> (u8, u8, u8) {
    let (mut r, mut g, mut b) = (0., 0., 0.);
    for i in x..x+size {
        for j in y..y+size {
            let temp = get_pixel(&buf, &image, i, j);
            let alpha = temp.3 as f64 / 255.;
            r += alpha * (temp.0 as f64);
            g += alpha * (temp.1 as f64);
            b += alpha * (temp.2 as f64);
        }
    }
    let size_2 = (size*size) as f64;
    r /= size_2;
    g /= size_2;
    b /= size_2;
    (r as u8, g as u8, b as u8)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Pass image as argument");
        std::process::exit(1);
    } else if args.len() > 2 {
        println!("Pass only image as argument");
        std::process::exit(1);
    }


    let image = image::open(&args[1]).unwrap();
    let buf = image.to_rgba().into_raw();       // todo unnecessary keeping two images in memory
    let scale = calc_downscale(&image);

    println!("Dimensions: {}x{}", image.width(), image.height());

    let mut last_y = 0; // todo ugly
    for y in (0..image.height() - 2 * scale + 1).step_by(scale as usize * 2) {
        for x in (0..image.width() - scale).step_by(scale as usize) {
            let (r, g, b) = sample(&buf, &image, x, y, scale);
            set_bg(r, g, b);
            let (r, g, b) = sample(&buf, &image, x, y + scale, scale);
            set_fg(r, g, b);
            print!("▄");
        }
        println!("{}", termion::style::Reset);
        last_y = y;
    }

    let last_y = last_y + 2 * scale;
    if last_y + scale < image.height() {
        for x in (0..image.width() - scale).step_by(scale as usize) {
            let (r, g, b) = sample(&buf, &image, x, last_y, scale);
            set_fg(r, g, b);
            print!("▀");
        }
    }

    println!("{}", termion::style::Reset);
}
