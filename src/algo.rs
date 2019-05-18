extern crate image;

use image::{ImageBuffer, Rgb};
use std::mem::swap;
use image::math::utils::clamp;

// draws line with Bresenham’s line algorithm
pub fn line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x0: i32, y0: i32, x1: i32, y1: i32, color: image::Rgb<u8>) {
    let mut steep = false;

    let mut xb = x0;
    let mut xe = x1;
    let mut yb = y0;
    let mut ye = y1;

    if i32::abs(x1 - x0) < i32::abs(y1 - y0) {
        xb = y0;
        yb = x0;
        xe = y1;
        ye = x1;
        steep = true;
    }

    if xb > xe {
        swap(&mut xb, &mut xe);
        swap(&mut yb, &mut ye);
    }

    println!("inner: ({}; {}) -> ({}; {})", xb, x1, yb, y1);
    let dx = xe - xb;
    let dy = ye - yb;
    let derr = i32::abs(dy) * 2;
    let mut err = 0;

    let mut x = xb;
    let mut y = yb;
    while x < xe {
        //println!("x {} y {}", x, y);
        x = clamp(x, 0, img.width() as i32 - 1);
        y = clamp(y, 0, img.height() as i32 - 1);
        if steep {
            img.put_pixel(y as u32, x as u32, color);
        } else {
            img.put_pixel(x as u32, y as u32, color);
        }
        err += derr;
        if err > dx {
            y += if yb > y0 { -1 } else { 1 };
            err -= dx * 2;
        }

        x += 1;
    }
    println!("------");
}