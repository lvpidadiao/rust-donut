#![feature(total_cmp)]

mod draw;
mod creeper;
mod canvas;
mod transform;
mod shapes;

use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use crate::canvas::Canvas;
use crate::draw::{PlainCircle, Point};
use crate::shapes::donut::Donut;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::sphere::Sphere;
use crate::transform::Animation;

fn main() {

    print!("\x1b[2J");
    print!("\x1b[?25l");// hide terminal cursor

    //let circle = PlainCircle::new(10.0, Point::origin());
    let rect = Rectangle::new(20, 20);

    let animator = Animation::new(0.2);

    let mut canvas = Canvas::new(40, 40, Some(animator));

    let mut sphere = Sphere::new(10.0);

    let mut torus = Donut::new(10, 6, 30, 35, 50, 50);

    //sphere.plot(40, 40);

    //canvas.buffer_2_screen_raw(&sphere.plot(40, 40, &[-1.0, 0.0, 0.0]));
    //canvas.buffer_2_screen_raw(&torus.regulated_pixels());
    let sleep_duration = 1000 / 30;
    let start_ts = Instant::now();



    loop {
        canvas.buffer_2_screen_raw(&torus.next_frame_with_x_rotate());
        sleep(Duration::from_millis(sleep_duration));

        if start_ts.elapsed().as_secs() >= 10 {
            break;
        }
    }


    //canvas.rotate(&rect);

    print!("\x1b[?25h");

    //println!("there are {} points in one calculate", circle.how_many_point());

    //sleep(Duration::from_secs(5));
}
