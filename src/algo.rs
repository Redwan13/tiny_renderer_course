extern crate image;

use image::{ImageBuffer, Rgb};
use std::mem::swap;
use image::math::utils::clamp;
use std::cmp::{min, max};

use super::types::{Vec2, Vec3};

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

// hardcoded impl. we don't need much more now
fn cross(v1: Vec3<f32>, v2: Vec3<f32>) -> Vec3<f32> {
    return Vec3::<f32> {
        x: v1.y * v2.z - v2.y * v1.z,
        y: v1.x * v2.z - v2.x * v1.z,
        z: v1.x * v2.y - v2.x - v1.y,
    };
}

fn barycentric_coords(p1: &Vec2<i32>, p2: &Vec2<i32>, p3: &Vec2<i32>, p: &Vec2<i32>) -> Vec3<f32> {
    let u = cross(
        Vec3::<f32> {
            x: (p3.x - p1.x) as f32,
            y: (p2.x - p1.x) as f32,
            z: (p1.x - p.x) as f32,
        },
        Vec3::<f32> {
            x: (p3.y - p1.y) as f32,
            y: (p2.y - p1.y) as f32,
            z: (p1.y - p.y) as f32,
        },
    );

    if f32::abs(u.z) < 1. {
        return Vec3::<f32> { x: -1., y: 1., z: 1. };
    }

    return Vec3::<f32> {
        x: 1. - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z,
    };
}

pub fn filled_triangle(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: &Vec2<i32>, p2: &Vec2<i32>, p3: &Vec2<i32>, color: image::Rgb<u8>) {
    let mut bbox_min = Vec2::<i32> { x: img.dimensions().0 as i32 - 1, y: img.dimensions().1 as i32 - 1 };
    let mut bbox_max = Vec2::<i32>::default();
    let clmp = bbox_min.clone();
    for p in vec! {p1, p2, p3} {
        bbox_min.x = max(0, min(p.x, bbox_min.x));
        bbox_max.x = min(clmp.x, max(p.x, bbox_max.x));

        bbox_min.y = max(0, min(p.y, bbox_min.y));
        bbox_max.y = min(clmp.y, max(p.y, bbox_max.y));
    }

    println!("bbox min w {} h {}", bbox_min.x, bbox_min.y);
    println!("bbox max w {} h {}", bbox_max.x, bbox_max.y);
    let mut p = Vec2::<i32>::default();
    for x in bbox_min.x..(bbox_max.x + 1) {
        for y in bbox_min.y..(bbox_max.y + 1) {
            p.x = x;
            p.y = y;

            let bary_point = barycentric_coords(p1, p2, p3, &p);
            println!("bary_point x {} y {} z {}", bary_point.x, bary_point.y, bary_point.z);
            if bary_point.x < 0. || bary_point.y < 0. || bary_point.z < 0. {
                continue;
            }

            println!("put pixel x {} y {}", x, y);
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}