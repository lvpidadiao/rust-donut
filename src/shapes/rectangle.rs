use std::collections::BTreeSet;
use crate::draw::Drawable;
use crate::Point;


pub struct Rectangle {
    width: i32,
    length: i32,
    point_buf: BTreeSet<Point>,
}


impl Rectangle {
    pub fn new(width: i32, length: i32) -> Self {
        let mut rect = Rectangle {
            width,
            length,
            point_buf: BTreeSet::new(),
        };

        rect.calc_point();

        return rect;
    }

    fn calc_point(&mut self) {
        for x in -self.width / 2 .. self.width / 2 + 1 {
            self.point_buf.insert(Point{x, y: -self.length / 2});
            self.point_buf.insert(Point{x, y: self.length / 2});
        }

        for y in -self.length / 2 .. self.length / 2 {
            self.point_buf.insert(Point{x: self.width / 2, y});
            self.point_buf.insert(Point{x: -self.width / 2, y});
        }

        self.point_buf.insert(Point::origin());
    }
}

impl Drawable for Rectangle {
    fn pixel_set(&self) -> &BTreeSet<Point> {
        return &self.point_buf;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_iter() {
        for i in -2..2 {
            print!("{}", i);
        }
        println!();
    }
}