use graphics::math::{Vec2d, add, mul_scalar};
use piston_window::{PistonWindow, WindowSettings, clear, rectangle};
use rand::distributions::{IndependentSample, Range};

type RGBA = [f32; 4];
const WHITE : RGBA = [1.0; 4];
const GRAY : RGBA = [0.7, 0.7, 0.7, 0.3];
const N_PARTICLES: usize = 500;

struct World{
    current_turn: usize,
    shapes: Vec<Box<Shape>>,
    height: u32,
    width: u32,
}

struct Shape {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: RGBA,
}

impl Shape {
    fn new(x: f64, y: f64) -> Shape{
        let mut rng = rand::thread_rng();
        let legal_range = Range::new(-5_f64, 5_f64);

        let x_speed = legal_range.ind_sample(&mut rng);
        let y_speed = legal_range.ind_sample(&mut rng);
        let x_accel = 0.1 * legal_range.ind_sample(&mut rng);
        let y_accel = 0.1 * legal_range.ind_sample(&mut rng);

        Shape{
            height: 10.0,
            width: 10.0,
            position: [x, y],
            velocity: [x_speed, y_speed],
            acceleration: [x_accel, y_accel],
            color : GRAY,
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.velocity, 0.7);
        self.color[3] *= 0.97;
    }
}

fn main() {
    println!("Hello, world!");
}
