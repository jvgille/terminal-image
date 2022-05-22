extern crate termion;
extern crate image;

use termion::color;

fn set_bg((r, g, b): (u8, u8, u8)) {
    print!("{}", color::Bg(color::Rgb(r, g, b)));
}

fn set_fg((r, g, b): (u8, u8, u8)) {
    print!("{}", color::Fg(color::Rgb(r, g, b)));
}

// calculates the downscaling factor necessary to fit the image inside the terminal
fn calc_downscale(width: u32, height: u32) -> u32 {
    let (t_width, t_height) = termion::terminal_size().unwrap();
    let h_scale = (width as f32/t_width as f32).ceil() as u32;
    let v_scale = (height as f32/(t_height*2) as f32).ceil() as u32;
    if h_scale > v_scale {
        h_scale
    } else {
        v_scale
    }
}

// gets pixel at x, y coordinate from rgba image buffer
fn get_pixel(buf: &Vec<u8>, image_width: u32, x: u32, y: u32) -> (u8, u8, u8, u8) {
    let base = ((y*image_width + x) * 4) as usize;
    (buf[base], buf[base+1], buf[base+2], buf[base+3])
}

// samples a square of size² elements and returns the average pixel, taking alpha into account
fn sample(buf: &Vec<u8>, image_width: u32, x: u32, y: u32, size: u32) -> (u8, u8, u8) {
    let (mut r, mut g, mut b) = (0., 0., 0.);
    for i in x..x+size {
        for j in y..y+size {
            let temp = get_pixel(&buf, image_width, i, j);
            let alpha = temp.3 as f32 / 255.;
            r += alpha * (temp.0 as f32);
            g += alpha * (temp.1 as f32);
            b += alpha * (temp.2 as f32);
        }
    }

    let size_2 = (size*size) as f32;
    r /= size_2;
    g /= size_2;
    b /= size_2;
    (r as u8, g as u8, b as u8)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Pass image as argument");
        std::process::exit(1);
    }

    let image = image::open(&args[1]).unwrap();
    let width = image.width();
    let height = image.height();
    let buf = image.into_rgba8().into_raw();
    let scale = calc_downscale(width, height);
    let rows = height / scale;
    let cols = width / scale;

    println!("Dimensions: {}x{}", width, height);

    for row in (0..rows).step_by(2) {
        for col in 0..cols {
            let y = row * scale;
            let x = col * scale;
            set_fg(sample(&buf, width, x, y, scale));
            if row != rows - 1 {
                // the last row for images with odd number of rows does not have a background colour
                set_bg(sample(&buf, width, x, y + scale, scale));
            }
            print!("▀");
        }
        println!("{}", termion::style::Reset);
    }
}
