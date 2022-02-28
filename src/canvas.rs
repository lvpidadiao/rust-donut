use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeSet;
use std::thread::sleep;
use std::time::Duration;
use crate::draw::Drawable;
use crate::Point;
use colored::*;
use crate::transform::Animation;

pub struct Canvas {
    length: i32,
    width: i32,
    x_offset: i32,
    y_offset: i32,
    animator: Option<Animation>,
}

impl Canvas {
    pub fn new(length: i32, width: i32, animator: Option<Animation>) -> Self {
        return Canvas{
            length,
            width,
            x_offset: width / 2,
            y_offset: length / 2,
            animator
        }
    }

    pub fn draw_shape<T: Drawable>(&self, something:& T) {
        //let draw_set = something.pixel_set();

        let mut last_x_cord_offset = 1;
        let orig_set = something.pixel_set();
        let mut draw_set = BTreeSet::<Point>::new();
        loop {
            // j is x , i is y
            draw_set.clear();
            for p in orig_set.iter() {
                draw_set.insert(Point{x: p.x + last_x_cord_offset,
                y: p.y});
            }

            last_x_cord_offset += 1;
            self.to_screen(&draw_set);
            sleep(Duration::from_secs(2));
        }
    }


    pub fn rotate<T>(&mut self,something:& T) where T: Drawable {
            let orig_set = something.pixel_set();
            let mut pixels = orig_set.clone();
            loop {
                self.to_screen(&pixels);
                pixels = self.animator.as_mut().unwrap().rotate_2d(orig_set);
                sleep(Duration::from_millis(1000 / 60));
            }

    }

    // fn offset_pixel(&self, buf: BTreeSet<Point>) {
    //
    // }


    fn to_screen(&self, pixels: &BTreeSet<Point>) {
        let mut draw_point = Point::origin();
        print!("\x1b[H");
        for y in 0..self.length {
            for x in 0..self.width {
                draw_point.x = x - self.x_offset;
                draw_point.y = y - self.y_offset;
                if pixels.contains(&draw_point) {
                    print!("{}", "*".red());
                }else {
                    print!(" ");
                }
                print!(" ");
            }
            print!("\n");
        }
    }
}