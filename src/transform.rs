use std::collections::BTreeSet;
use std::f32::consts::PI;
use std::ops::{Mul, Rem};
use crate::draw::Drawable;
use crate::Point;

pub struct Animation {
    resolution: f32,
    step: i32,
    interval: i32,
    theta: f32,
    two_pi: f32,
}


impl Animation {

    pub fn new(res: f32) -> Self {
        Animation {
            resolution: res, // 0.2
            step: 1,
            interval: 1,
            theta: res,
            two_pi: PI.mul(2.0),
        }
    }

    pub fn rotate_2d(&mut self, shape_points: &BTreeSet<Point>) -> BTreeSet<Point> {
        let mut set = BTreeSet::new();
        let mut x1 = 0f32;
        let mut y1 = 0f32;

        for p in shape_points {
            let (sin, cos) = self.theta.sin_cos();
            x1 = p.x as f32 * cos - p.y as f32 * sin;
            y1 = sin * p.x as f32 + cos * p.y as f32;
            set.insert(Point{x: x1.round() as i32, y: y1.round() as i32});
        }


        self.theta += self.resolution;
        self.theta = self.theta.rem(self.two_pi);

        return set;
    }

    pub fn rotate2d_offset(&mut self, shape_points: &BTreeSet<Point>, offset: i32) -> BTreeSet<Point> {
        let mut set = BTreeSet::new();
        let mut x1 = 0f32;
        let mut y1 = 0f32;

        for p in shape_points {
            let (sin, cos) = self.theta.sin_cos();
            x1 = p.x as f32 * cos - p.y as f32 * sin;
            y1 = sin * p.x as f32 + cos * p.y as f32;
            set.insert(Point{x: x1.round() as i32 + offset, y: y1.round() as i32 + offset});
        }


        self.theta += self.resolution;
        self.theta = self.theta.rem(self.two_pi);

        return set;
    }
}



#[cfg(test)]
mod test {
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{Cell, RefCell};
    use std::collections::{BTreeSet, HashMap};
    use super::*;

    #[test]
    fn test1() {

    }
}