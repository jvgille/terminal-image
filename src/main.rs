extern crate png;
extern crate termion;

use termion::color;
use std::fs::File;

fn set_bg(c: color::Rgb) {
    print!("{}", color::Bg(c));
}

fn set_fg(c: color::Rgb) {
    print!("{}", color::Fg(c));
}

fn to_string(b: &png::BitDepth) -> &str {
    match b {
        png::BitDepth::One => "1",
        png::BitDepth::Two => "2",
        png::BitDepth::Four => "4",
        png::BitDepth::Eight => "8",
        png::BitDepth::Sixteen => "16",
    }
}

fn calc_downscale(info: &png::OutputInfo) -> usize {
    let (t_width, t_height) = termion::terminal_size().unwrap();
    let (t_width, t_height) = (t_width as f64, (t_height*2) as f64);
    let h_scale = (info.width as f64/t_width).ceil();
    let v_scale = (info.height as f64/t_height).ceil();
    if h_scale > v_scale {
        h_scale as usize
    } else {
        v_scale as usize
    }
}

fn get_pixel(buf: &Vec<u8>, info: &png::OutputInfo, x: u32, y: u32) -> (u8, u8, u8, u8) {
    let base = ((y*info.width + x) * (info.color_type.samples() as u32)) as usize;
    (buf[base], buf[base+1], buf[base+2], buf[base+3])
}

fn sample(buf: &Vec<u8>, info: &png::OutputInfo, x: u32, y: u32, size: u32) -> (u8, u8, u8) {
    let (mut r, mut g, mut b) : (f64, f64, f64) = (0., 0., 0.);
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

    println!("Bit depth: {}\nSamples: {}\nDimensions: {}x{}",
            to_string(&info.bit_depth),
            info.color_type.samples(),
            info.width, info.height);

    let mut last_y = 0;
    for y in (0..info.height - 2 * scale as u32 + 1).step_by(scale*2) {
        for x in (0..info.width - scale as u32).step_by(scale) {
            let (r, g, b) = sample(&buf, &info, x, y, scale as u32);
            set_bg(color::Rgb(r, g, b));
            let (r, g, b) = sample(&buf, &info, x, y + scale as u32, scale as u32);
            set_fg(color::Rgb(r, g, b));
            print!("▄");
        }
        println!("{}", termion::style::Reset);
        last_y = y;
    }

    for x in (0..info.width - scale as u32).step_by(scale) {
        let y = last_y + 2 * scale as u32;
        let (r, g, b) = sample(&buf, &info, x, y, scale as u32);
        set_fg(color::Rgb(r, g, b));
        print!("▀");
    }

    println!("{}", termion::style::Reset);
}
