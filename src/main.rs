extern crate png;
extern crate termion;

use termion::color;
use std::fs::File;

fn set_bg(r : u8, g : u8, b : u8) {
    print!("{}", color::Bg(color::Rgb(r, g, b)));
}

fn set_fg(r : u8, g : u8, b : u8) {
    print!("{}", color::Fg(color::Rgb(r, g, b)));
}

fn calc_downscale(info: &png::OutputInfo) -> u32 {
    let (t_width, t_height) = termion::terminal_size().unwrap();
    let (t_width, t_height) = (t_width as f64, (t_height*2) as f64);
    let h_scale = (info.width as f64/t_width).ceil();
    let v_scale = (info.height as f64/t_height).ceil();
    if h_scale > v_scale {
        h_scale as u32
    } else {
        v_scale as u32
    }
}

fn get_pixel(buf: &Vec<u8>, info: &png::OutputInfo, x: u32, y: u32) -> (u8, u8, u8, u8) {
    let base = ((y*info.width + x) * (info.color_type.samples() as u32)) as usize;
    (buf[base], buf[base+1], buf[base+2], buf[base+3])
}

fn sample(buf: &Vec<u8>, info: &png::OutputInfo, x: u32, y: u32, size: u32) -> (u8, u8, u8) {
    let (mut r, mut g, mut b) = (0., 0., 0.);
    for i in x..x+size {
        for j in y..y+size {
            let temp = get_pixel(&buf, &info, i, j);
            let alpha = temp.3 as f64 / 255 as f64;
            r += alpha * (temp.0 as f64);
            g += alpha * (temp.1 as f64);
            b += alpha * (temp.2 as f64);
        }
    }
    r /= (size*size) as f64;
    g /= (size*size) as f64;
    b /= (size*size) as f64;
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

    let decoder = png::Decoder::new(File::open(&args[1]).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let scale = calc_downscale(&info);

    println!("Dimensions: {}x{}", info.width, info.height);

    let mut last_y = 0;
    for y in (0..info.height - 2 * scale + 1).step_by(scale as usize * 2) {
        for x in (0..info.width - scale).step_by(scale as usize) {
            let (r, g, b) = sample(&buf, &info, x, y, scale);
            set_bg(r, g, b);
            let (r, g, b) = sample(&buf, &info, x, y + scale, scale);
            set_fg(r, g, b);
            print!("▄");
        }
        println!("{}", termion::style::Reset);
        last_y = y;
    }

    let last_y = last_y + 2 * scale;
    if last_y + scale < info.height {
        for x in (0..info.width - scale).step_by(scale as usize) {
            let (r, g, b) = sample(&buf, &info, x, last_y, scale);
            set_fg(r, g, b);
            print!("▀");
        }
    }

    println!("{}", termion::style::Reset);
}
