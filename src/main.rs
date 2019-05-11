extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::{ImageBuffer, Rgb};

mod algo;

fn prepare_canvas(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }
}

fn lesson1(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    println!("lesson 1. Bresenham’s line algorithm");

    algo::line(img, 80, 80, 120, 120, image::Rgb([255,255,255]));
    algo::line(img, 100, 110, 130, 130, image::Rgb([255,0,0]));

    algo::line(img, 140, 100, 100, 150, image::Rgb([0,0,255]));
}


fn main() {
    let matches = App::new("Tiny renderer course implementation")
        .arg(Arg::with_name("output").index(1).help("output").required(true)).get_matches();

    let out = matches.value_of("output");
    println!("output: {}", out.unwrap());

    let mut img = image::ImageBuffer::new(200, 200);
    prepare_canvas(&mut img);
    lesson1(&mut img);

    img.save(out.unwrap()).expect("Couldn't save image!");
}
