use std::cmp::Ordering;
use std::collections::{HashSet, BTreeSet};

pub trait Drawable{
    fn pixel_set(&self) -> &BTreeSet<Point>;
}

#[derive(PartialOrd, Ord, Eq, Copy, Clone)]
pub struct Point{
    pub x: i32,
    pub y: i32,
}

// impl Eq for Point {
//
// }

impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

// impl Ord for  Point{
//     fn cmp(&self, other: &Self) -> Ordering {
//         return if self.x.eq(&other.x) && self.y.eq(&other.y) {
//             Ordering::Equal
//         } else {
//             if self.x.eq(&other.x) {
//                 self.y.partial_cmp(&other.y).unwrap()
//             } else {
//                 self.x.partial_cmp(&other.x).unwrap()
//             }
//         }
//     }
//
//     fn max(self, other: Self) -> Self where Self: Sized {
//         self
//     }
//
//     fn min(self, other: Self) -> Self where Self: Sized {
//         self
//     }
//
//     fn clamp(self, min: Self, max: Self) -> Self where Self: Sized {
//         self
//     }
// }

pub struct PlainCircle {
    origin: Point,
    radius: f32,
    point_buf: BTreeSet<Point>,
    resolution: f32
}

impl Drawable for PlainCircle {
    fn pixel_set(&self) -> &BTreeSet<Point> {
        return &self.point_buf;
    }
}

impl Point {
    pub fn origin() -> Self {
        Point{
            x:0,
            y:0,
        }
    }
}


impl PlainCircle {
    pub fn new(radius: f32, orig: Point) -> Self {
        let mut circle = PlainCircle{
            origin: orig,
            radius,
            point_buf: BTreeSet::new(),
            resolution: 0.01,
        };
        circle.cal_point();
        return circle;
    }


    fn cal_point(&mut self) {
        let mut theta = self.resolution;
        let twoPI = std::f32::consts::PI * 2.0;
        self.point_buf.clear();
        loop {
            if theta > twoPI {
                break;
            }

            let (y1, x1) = theta.sin_cos();
            let rx1 = (x1 * self.radius).round() as i32 ;
            let ry1 = (y1 * self.radius).round() as i32;
            //println!("insert point ({}, {})", rx1, ry1);
            self.point_buf.insert(Point{x: rx1, y: ry1});
            theta += self.resolution;
        }
    }

    pub fn how_many_point(&self) -> usize {
        return self.point_buf.len();
    }
}

struct Creeper {

}