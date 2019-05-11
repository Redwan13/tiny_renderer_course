extern crate image;

use image::{ImageBuffer, Rgb};
use std::mem::swap;

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

    let dx = xe - xb;
    let dy = ye - yb;
    let derr = i32::abs(dy) * 2;
    let mut err = 0;

    let mut x = xb;
    let mut y = yb;
    while x < xe {
        if steep {
            img.put_pixel(y as u32, x as u32, color);
        } else {
            img.put_pixel(x as u32, y as u32, color);
        }
        err += derr;
        if err > dx {
            y += if yb > y0 {-1} else {1};
            err -= dx * 2;
        }

        x += 1;
    }
}