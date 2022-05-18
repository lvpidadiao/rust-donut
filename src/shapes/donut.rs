use std::f32::consts::PI;

pub struct Donut {
    r1: u32,
    r2: u32,
    viewer_distance: u32,
    object_distance: u32,
    scr_width: usize,
    scr_height: usize,
    psi: f32,
    delta: f32,
}


impl Donut {
    pub fn new(r1: u32, r2: u32, vd: u32, od: u32, sw: usize, sl: usize) -> Self {
        Donut {
            r1,
            r2,
            viewer_distance: vd,
            object_distance: od,
            scr_width: sw,
            scr_height: sl,
            psi: 0.0,
            delta: 0.0,
        }
    }

    // zx平面画圆，围绕z轴转动
    // r2*cos(psi)*cos(the) + r1*cos(psi), -r2*sin(psi)*cos(the) - r1*sin(psi), r2*sin(the)
    pub fn regulated_pixels(&self) -> Vec<Vec<char>> {
        let mut zbuf = vec![vec![-123;self.scr_height];self.scr_width];
        let mut output = vec![vec!['\0';self.scr_height];self.scr_width];
        let mut psi = 0.0f32;
        let mut theta = 0.0f32;
        let mut delta = 0.0f32;
        let two_pi = PI * 2.0;

        let wid_offset = (self.scr_width / 2) as f32;
        let hei_offset = (self.scr_height / 2) as f32;

        loop {
            psi += 0.05;
            let (sin_p, cos_p) = psi.sin_cos();
            let z = (self.r2 as f32 * sin_p) as i32;
            let z_apo = self.viewer_distance as f32 / (self.object_distance as f32 - z as f32);

            loop {
                theta += 0.05;

                let (sin_t, cos_t) = theta.sin_cos();

                let x = self.r2 as f32 * cos_t * cos_p + self.r1 as f32 * cos_t;
                let y = -1.0 * self.r2 as f32 * sin_t * cos_p - self.r1 as f32 * sin_t;
                let xp = (x * z_apo + wid_offset) as usize;
                let yp = (y * z_apo + hei_offset) as usize;


                if z > zbuf[xp][yp] {
                    zbuf[xp][yp] = z;
                    output[xp][yp] = '*';
                }

                if theta.ge(&two_pi) {
                    break;
                }
            }
            theta = 0.0;
            if psi.ge(&two_pi) {
                break;
            }
        }
        return output;
    }


    // torus rotate along with y-axis counter-clockwise
    // [-r2*sin(delta)*sin(the) + r2*cos(delta)*cos(psi)*cos(the) + r1*cos(delta)*cos(psi), -r2*sin(psi)*cos(the) - r1*sin(psi), r2*sin(delta)*cos(psi)*cos(the) + r1*sin(delta)*cos(psi) + r2*sin(the)*cos(delta)]
    pub fn next_frame(&mut self) -> Vec<Vec<char>> {
        let mut zbuf = vec![vec![-123;self.scr_height];self.scr_width];
        let mut output = vec![vec!['\0';self.scr_height];self.scr_width];
        let mut psi = 0.0f32;
        let mut theta = 0.0f32;
        let mut delta = 0.0f32;
        let two_pi = PI * 2.0;

        let wid_offset = (self.scr_width / 2) as f32;
        let hei_offset = (self.scr_height / 2) as f32;

        self.delta += 0.17;
        let (sin_d, cos_d) = self.delta.sin_cos();

        loop {
            psi += 0.05;
            loop {
                theta += 0.05;
                let (sin_t, cos_t) = theta.sin_cos();
                let (sin_p, cos_p) = psi.sin_cos();
                let z = (self.r2 as f32 * sin_d * cos_p * cos_t + self.r1 as f32 * sin_d * cos_p + self.r2 as f32 * sin_t * cos_d) as i32;
                let z_apo = self.viewer_distance as f32 / (self.object_distance as f32 - z as f32);


                let x = -1.0 * self.r2 as f32 * sin_d * sin_t + self.r2 as f32 * cos_t * cos_p * cos_d + self.r1 as f32 * cos_d * cos_p;
                let y = -1.0 * self.r2 as f32 * sin_p * cos_t - self.r1 as f32 * sin_p;
                let xp = (x * z_apo + wid_offset) as usize;
                let yp = (y * z_apo + hei_offset) as usize;


                if z > zbuf[xp][yp] {
                    zbuf[xp][yp] = z;
                    output[xp][yp] = '*';
                }

                if theta.ge(&two_pi) {
                    break;
                }
            }
            theta = 0.0;
            if psi.ge(&two_pi) {
                break;
            }
        }
        return output;
    }

}