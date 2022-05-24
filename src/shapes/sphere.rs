use std::f32::consts::PI;
use std::mem::size_of;
use std::ops::Rem;

pub static LUMINANCE_CHARS: [char; 12] = ['.', ',', '-','~',':', ';', '=', '!', '*', '#', '$', '@'];


pub struct Sphere {
    radius: f32,
    resolution: f32,
    viewer_distance: i32,
    object_distance: i32,
    theta: f32,
    psi: f32,
}

impl Sphere {

}


pub fn dot_product_3d(a: [f32;3], b: &[f32;3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        let k1 = 30;
        let k2 = 0;
        Sphere{
            radius,
            resolution: 0.17,
            viewer_distance: k1,
            object_distance: k1 + k2,
            theta: 0.0,
            psi: 0.0,
        }
    }

    pub fn plot(&mut self, screen_width: usize, screen_height: usize, light_source: &[f32;3]) -> Vec<Vec<char>> {
        // depth value
        // convert it to luminance value
        // zbuf[x][y] present a point
        let r_integer = self.radius as usize + 1;
        let mut zbuf = vec![vec![-123;screen_width];screen_height];
        let mut output = vec![vec!['\0';screen_width];screen_height];
        let two_pi = PI*2.0;
        //let resize_z_factor = 128.0 / self.radius;

        // rotate counter-clockwise at x-y plane
        loop {
            self.theta += 0.1;
            let (sin_the, cos_the) = self.theta.sin_cos();
            let z = (self.radius * cos_the) as i32;
          //  let z_apostrophe = char::from_u32((z * resize_z_factor) as u32).unwrap();
            loop {
                self.psi += 0.1;
                let (sin_psi, cos_psi) = self.psi.sin_cos();


                let x=  (self.radius * sin_the * cos_psi  ) as i32;
                let y = (self.radius * sin_the * sin_psi  ) as i32;
                let xp = ((x * self.viewer_distance) / (self.object_distance - z)) + screen_width as i32 / 2;
                let yp = ((y * self.viewer_distance) / (self.object_distance - z)) + screen_height as i32 / 2;
                let xc = xp as usize;
                let yc = yp as usize;
                if z > zbuf[xc][yc] {
                    zbuf[xc][yc] = z;
                    // calculate luminance, and project to xp, yp,
                    // make x projection, y projection


                    // calculate luminance index
                    // first find normal, as a sphere, surface normal is just [x, y, z]
                    // let light source as [-1, 1, 1]
                    // dot product
                    let mut lum = dot_product_3d([cos_psi, sin_psi , cos_the], light_source);
                    //lum += 2.1;
                    if lum < 0.0 {
                        continue;
                    }

                    let lum_index = (lum * 11.4) as usize ;
                    //let lum_index = 7;
                    if output[xc][yc] != '\0' {
                        println!("[{},{}] has been set before. original char {}, new char {}", xc, yc,
                                 output[xc][yc], LUMINANCE_CHARS[lum_index]);
                    }
                    output[xc][yc] = LUMINANCE_CHARS[lum_index];
                }

                if self.psi.ge(&two_pi) {
                    break;
                }
            }
            self.psi = 0.0;
            if self.theta.ge(&two_pi) {
                break;
            }
        }


        return output;
    }


    //todo: 对于球体，旋转球体从视觉上并不能分辨。可以旋转光源，变换打光方位，观察球体。
    // pub fn just_draw_a_zbuf(&mut self) -> Vec<Vec<usize>>{
    //
    // }



}


