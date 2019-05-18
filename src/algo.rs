extern crate image;

use image::{ImageBuffer, Rgb};
use std::mem::swap;
use image::math::utils::clamp;

use super::types::Vec2;

// draws line with Bresenham’s line algorithm
pub fn line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, begin: &Vec2<i32>, end: &Vec2<i32>, color: image::Rgb<u8>) {
    let mut steep = false;

    let mut xb = begin.x;
    let mut xe = end.x;
    let mut yb = begin.y;
    let mut ye = end.y;

    if i32::abs(end.x - begin.x) < i32::abs(end.y - begin.y) {
        xb = begin.y;
        yb = begin.x;
        xe = end.y;
        ye = end.x;
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
        x = clamp(x, 0, img.width() as i32 - 1);
        y = clamp(y, 0, img.height() as i32 - 1);
        if steep {
            img.put_pixel(y as u32, x as u32, color);
        } else {
            img.put_pixel(x as u32, y as u32, color);
        }
        err += derr;
        if err > dx {
            y += if yb > begin.y { -1 } else { 1 };
            err -= dx * 2;
        }

        x += 1;
    }
}

pub fn triangle(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: &Vec2<i32>, p2: &Vec2<i32>, p3: &Vec2<i32>, color: image::Rgb<u8>) {
    line(img, p1, p2, color);
    line(img, p2, p3, color);
    line(img, p3, p1, color);
}