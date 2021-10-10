use log::info;
use std::cmp;
use std::ops::Add;
use rand::prelude::*;
use web_sys::{CanvasRenderingContext2d, CanvasGradient};
use wasm_bindgen::{JsValue};

#[derive(Debug, Clone, PartialEq)]
pub struct LavaLamp {
    step: f32,
    width: f32,
    height: f32,
    wh: f32,
    sx: f32,
    sy: f32,
    paint: bool,
    meta_fill: CanvasGradient,
    plx: Vec<i8>,
    ply: Vec<i8>,
    mscases: Vec<f32>,
    ix: Vec<f32>,
    grid: Vec<Point>,
    balls: Vec<Ball>,
    iter: f32,
    sign: f32,
    ctx: CanvasRenderingContext2d,
    num_balls: usize,
    col0: String,
    col1: String,
}

impl LavaLamp {
    pub fn new(w: f32, h: f32, nb: usize, c0: String, c1: String, ctx: Option<CanvasRenderingContext2d>) -> Self {

        let ctx = ctx.unwrap();
        let stp = 10.0;
        let cg = create_radial_gradient(w, h, c0.clone(), c1.clone(), &ctx);
        let sx = (w / stp).floor();
        let sy = (h/stp).floor();
        let wh =  w.min(h);

        let mut lamp = LavaLamp {
            step: stp,
            width: w,
            height: h,
            wh,
            sx,
            sy,
            paint: false,
            meta_fill: cg,
            plx: vec![0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0],
            ply: vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1],
            mscases: vec![0.0, 3.0, 0.0, 3.0, 1.0, 3.0, 0.0, 3.0, 2.0, 2.0, 0.0, 2.0, 1.0, 1.0, 0.0],
            ix: vec![1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0],
            grid: vec![],
            balls: vec![],
            iter: 0.0,
            sign: 1.0,
            ctx,
            num_balls: nb,
            col0: c0,
            col1: c1,
        };

        let grid_limit = ((sx + 2.0) * (sy + 2.0)) as usize;
        for i in 0..grid_limit {
            let px = ( i as f32 % (sx + 2.0)) * stp;
            let py = (i as f32 / (sx + 2.0)).floor() * stp;
            let point = Point {
                x: px,
                y: py,
                magnitude: (px * px) + (py * py),
                ..Default::default()
            };
            lamp.grid.push(point);
        }

        for i in 0..nb {
            let ball = Ball::new(w, h, wh);
            lamp.balls.push(ball);
        }
        lamp
    }

    fn compute_force(&mut self, x: f32, y: f32, idx:f32) -> f32 {
        
        let mut force: f32 = 0.0;
        
        if x == 0.0 || y == 0.0 || x == self.sx || y == self.sy {
            force = 0.6 * self.sign;
        } else {
            let cell = if idx as usize > self.grid.len() {
                self.grid.last().unwrap()
            } else{
                &self.grid[idx as usize]
            };
            let mut i = 0;
            while i < self.balls.len() {
                let den =(
                    -2.0 * cell.x * self.balls[i].pos.x - 2.0 * cell.y * self.balls[i].pos.y + self.balls[i].pos.magnitude + cell.magnitude);

                force += (
                    (self.balls[i].size * self.balls[i].size)
                    /
                    den) as f32;

            i += 1;
            }
            force *= self.sign;

        }

        if idx as usize > self.grid.len() {
            self.grid.last().unwrap().to_owned().force = force;
        } else{
            self.grid[idx as usize].force = force;
        }
        force
    }

    fn marching_squares(&mut self, x: f32,y: f32, p_dir: f32) -> Option<(f32, f32, f32)> {

        let id = (x + y * (self.sx + 2.0)).floor() as usize;

        if self.grid[id].computed == self.iter {
            return None;
        }
        
        let mut force: f32;
        let mut ms_case: f32 = 0.0;
        let mut dir: f32 = if p_dir == -1.0 { 0.0 } else { p_dir };


        for i in 0..4 {
            let idn = (x + self.ix[i + 12]) + (y + self.ix[i + 16]) * (self.sx + 2.0);
            force = if idn as usize > self.grid.len() {
                self.grid.last().unwrap().force

            } else {self.grid[idn as usize].force};

            if (force > 0.0 && self.sign < 0.0) ||
                (force < 0.0 && self.sign > 0.0 || force == 0.0) {
                force = self.compute_force(
                    x + self.ix[i + 12],
                    y + self.ix[i + 16],
                    idn
                );
                
            }
            if force.abs() > 1.0 {
                ms_case += (2u8.pow(i as u32)) as f32; 
            }
        }

        if ms_case == 15.0 {
            return Some((x, y - 1.0, -1.0));
        } else {

            if ms_case == 5.0 {
                dir =  if dir == 2.0 { 3.0} else {1.0};
            } else if ms_case == 10.0 {
                dir =  if dir == 3.0 {0.0} else { 2.0};
            } else {
                dir = self.mscases[ms_case as usize];
                self.grid[id].computed = self.iter;
            }
            
            let mut g_ind = ((x + self.plx[(4.0 * dir + 3.0) as usize] as f32) +
                            (y + self.ply[(4.0 * dir + 3.0) as usize] as f32) *
                            (self.sx + 2.0)) as usize;
            if g_ind > self.grid.len() {
                g_ind = self.grid.len() - 1;
            }

            let ix = self.step / 
                (
                    ((
                        self.grid[(
                            (x + self.plx[(4.0 * dir + 2.0) as usize] as f32) +
                            (y + self.ply[(4.0 * dir + 2.0) as usize] as f32) *
                            (self.sx + 2.0)
                            ) as usize 
                        ].force
                    ).abs()
                    - 1.0
                    ).abs()
                    / 
                    (
                        (self.grid[g_ind].force).abs() - 1.0
                    ).abs() + 1.0
                );


                let lxx =  self.grid[(
                        (x + self.plx[(4.0 * dir) as usize] as f32) +
                        (y + self.ply[(4.0 * dir) as usize] as f32) *
                        (self.sx + 2.0)
                    ) as usize];
                let lix = self.ix[dir as usize] * ix;
                let lx = 
                    (lxx.x
                    +
                    lix) as f64
                    ;

                let mut ly_ind = (
                        (x + self.plx[(4.0 * dir + 1.0) as usize] as f32) +
                        (y + self.ply[(4.0 * dir + 1.0) as usize] as f32) *
                        (self.sx + 2.0)
                    ) as usize;
                if ly_ind > self.grid.len() {
                    ly_ind = self.grid.len() -1;
                } 
                let ly =  (self.grid[ly_ind].y +
                    (self.ix[(dir + 4.0) as usize] as f32 * ix)) as f64;

                self.ctx.line_to(
                    lx
                    ,
                    ly
                    );
                self.paint = true;
                let new_next = Some((
                    (x + self.ix[(dir + 4.0) as usize] as f32),
                    (y + self.ix[(dir + 8.0) as usize] as f32),
                    dir 
                ));

                return new_next;
        }
    }

    pub fn render_metaball(&mut self) {
        // self.ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        for ball in self.balls.iter_mut() {
            ball.mov();
        }
        self.iter += 1.0;
        self.sign = -1.0 * self.sign;
        self.paint = false;

        self.ctx.set_fill_style(&JsValue::from(self.meta_fill.to_owned()));
        self.ctx.begin_path();

            self.ctx.set_shadow_blur(50.0);
            self.ctx.set_shadow_color("black");

            for ball in  self.balls.clone().iter() {
                let mut next = Some((
                    (ball.pos.x / self.step).round(),
                    (ball.pos.y / self.step).round(),
                    -1.0
                ));
                loop {
                    next = self.marching_squares(
                        next.unwrap().0,
                        next.unwrap().1,
                        next.unwrap().2
                    );

                    if next.is_none() { break; }
                }

                //let march = self.marching_squares(next.0, next.1, next.2);


                if self.paint {
                    self.ctx.fill();
                    // self.ctx.set_stroke_style(&JsValue::from("orange"));
                    // self.ctx.stroke();
                    self.ctx.close_path();
                    self.ctx.begin_path();
                    self.paint = false;
                }
            }

         // self.ctx.set_fill_style(&JsValue::from("red"));
         // self.ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

    }

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    magnitude: f32,
    computed: f32,
    force: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ball {
    vel: Point,
    pos: Point,
    size: f32,
    width: f32,
    height: f32,
}


impl Default for Point{
    fn default() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            magnitude: 0.0,
            computed: 0.0,
            force: 0.0,
        }
    }
}

impl Ball {
    fn new(w: f32, h: f32, wh: f32) -> Self {
        let mut rng = thread_rng();
        let rx: f64 = rng.gen();
        let ry: f64 = rng.gen();
        let rx2: f64 = rng.gen();
        let ry2: f64 = rng.gen();
        // let rx: f64 = 0.2;
        // let ry: f64 = 0.2;
        // let rx2: f64 = 0.2;
        // let ry2: f64 = 0.2;


        let px = (if rx > 0.5f64 {1.0f64} else {-1.0f64}) * (0.2 + rx2 * 0.25f64);
        let py = (if ry > 0.5f64 {1.0f64} else {-1.0f64}) * (0.2 + ry);

        let p_vel = Point{
            x: px as f32,
            y: py as f32,
            magnitude: (px * px) as f32 + (py * py) as f32,
            ..Default::default()
        };

        let pos_x = (w * 0.2) + (rx2 as f32 * w * 0.6);
        let pos_y =  (h * 0.2) + (ry2 as f32 * h * 0.6);


        let p_pos =  Point {
            x: pos_x, 
            y: pos_y,
            magnitude: (pos_x * pos_x) + (pos_y * pos_y),
            ..Default::default()
        };

        let size = (wh/15.0) + rx as f32 * (wh / 15.0);

        Ball {
            vel: p_vel,
            pos: p_pos,
            size,
            width: w,
            height: h,
        }
    }

    fn mov(&mut self) {
       if self.pos.x >= (self.width - self.size) {
           if self.vel.x > 0.0 {
               self.vel.x = -1.0 * self.vel.x;
           }
           self.pos.x = self.width - self.size;
       } else if self.pos.x <= self.size {
            if self.vel.x < 0.0 {
                self.vel.x = -1.0 * self.vel.x;
            }
            self.pos.x = self.size;
       }

       if self.pos.y >= (self.height - self.size) {
           if self.vel.y > 0.0 {
               self.vel.y = -1.0 * self.vel.y;
           }
           self.pos.y = self.height - self.size;
       } else if self.pos.y <= self.size {
           if self.vel.y < 0.0 {
               self.vel.y = -1.0 * self.vel.y;
           }
           self.pos.y = self.size;
       }

       self.pos = self.pos.add(self.vel);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_x = self.x + other.x;
        let new_y = self.y + other.y;
        Self {
            x: new_x,
            y: new_y,
            magnitude: (new_x * new_x) + (new_y * new_y),
            ..self
        }
    }
}


fn create_radial_gradient(w: f32, h: f32, c0: String, c1: String, ctx: &CanvasRenderingContext2d) -> CanvasGradient {
        let gradient = ctx.create_radial_gradient(
                (w/2.0) as f64, (h/2.0) as f64, 0.0 as f64,
                (w/2.0) as f64, (h/2.0) as f64, (w/2.0) as f64
            ).unwrap();
        gradient.add_color_stop(0.0, &c0);
        gradient.add_color_stop(1.0, &c1);
        gradient
    }
