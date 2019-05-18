extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::{ImageBuffer, Rgb};

use std::fs::File;
use std::io::BufReader;
use obj::raw::{parse_obj, RawObj};
use obj::raw::object::Polygon;
use std::cmp::max;

mod algo;

fn prepare_canvas(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }
}

fn lesson1(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    println!("lesson 1. Bresenham’s line algorithm");

    let input = BufReader::new(File::open("resources/head.obj").unwrap());
    let dome: RawObj = parse_obj(input).unwrap();

    let (width, height) = img.dimensions();

    let mut current = -1.;
    let len = dome.polygons.len() as f32;
    let mut r = 255;
    let mut g = 255;
    let mut b = 255;
    for poly in dome.polygons.iter().rev() {
        current += 1.;
        let idxs = match &poly {
            Polygon::PTN(v) => v,
            _ => {
                continue;
            }
        };
//        let step = (255. * current / len) as u8;
//        if r > 0 {
//            r = max(r - step as u8, 0);
//        }
//        if r == 0 && g > 0 {
//            g = max(g - step, 0);
//        }
//        if r == 0 && g == 0 && b > 0 {
//            b = max(b - step, 0);
//        }

        //println!("r {} g {} b {}", r, g, b);
        let color = image::Rgb([r, g, b]);
        for j in 0..3 {
            let v0 = dome.positions[idxs[j].0 as usize];
            let v1 = dome.positions[idxs[((j + 1) % 3)].0 as usize]; // enclose the triangle
            let x0 = (v0.0 + 1.) * width as f32 / 2.;
            let y0 = (v0.1 + 1.) * height as f32 / 2.;
            let x1 = (v1.0 + 1.) * width as f32 / 2.;
            let y1 = (v1.1 + 1.) * height as f32 / 2.;
            println!("{} -> {}:  ({}; {}) -> ({}; {})", idxs[j].0, idxs[(j + 1) % 3].0, x0, y0, x1, y1);
            algo::line(img, (width - (x0 as u32)) as i32, (height - y0 as u32) as i32, (width - x1 as u32) as i32, (height - y1 as u32) as i32, color);
        }
    }
}


fn main() {
    let matches = App::new("Tiny renderer course implementation")
        .arg(Arg::with_name("output").index(1).help("output").required(true)).get_matches();

    let out = matches.value_of("output");
    println!("output: {}", out.unwrap());

    let mut img = image::ImageBuffer::new(800, 800);
    prepare_canvas(&mut img);
    println!("Prepared canvas");
    lesson1(&mut img);

    println!("Completed calculations");
    img.save(out.unwrap()).expect("Couldn't save image!");
}
