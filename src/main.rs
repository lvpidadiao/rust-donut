#![feature(total_cmp)]

mod draw;
mod creeper;
mod canvas;
mod transform;
mod shapes;

use std::thread::sleep;
use std::time::Duration;
use crate::canvas::Canvas;
use crate::draw::{PlainCircle, Point};
use crate::shapes::donut::Donut;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::sphere::Sphere;
use crate::transform::Animation;

fn main() {

    print!("\x1b[2J");
    //print!("\x1b[?25l");// hide terminal cursor

    //let circle = PlainCircle::new(10.0, Point::origin());
    let rect = Rectangle::new(20, 20);

    let animator = Animation::new(0.2);

    let mut canvas = Canvas::new(40, 40, Some(animator));

    let mut sphere = Sphere::new(10.0);

    let mut torus = Donut::new(10, 3, 30, 35, 40, 40);

    //sphere.plot(40, 40);

    //canvas.buffer_2_screen_raw(&sphere.plot(40, 40, &[-1.0, 0.0, 0.0]));
    //canvas.buffer_2_screen_raw(&torus.regulated_pixels());
    let sleep_duration = 1000 / 24;

    loop {
        canvas.buffer_2_screen_raw(&torus.next_frame());
        sleep(Duration::from_millis(sleep_duration));
    }


    //canvas.rotate(&rect);

    //print!("\x1b[?25h");

    //println!("there are {} points in one calculate", circle.how_many_point());

    sleep(Duration::from_secs(5));
}
