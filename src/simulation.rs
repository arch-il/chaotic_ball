use std::f32::consts::PI;

use macroquad::{
    color,
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines},
};

pub struct Simulation {
    pub balls: Vec<Ball>,
    pub radius: f32,
    pub outer_radius: f32,
}

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            balls: vec![Ball {
                pos: Vec2::new(300.0, 250.0),
                vel: Vec2::ZERO,
            }],
            radius: 10.0,
            outer_radius: 200.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        const STEP_SIZE: f32 = 0.001;
        for _ in 0..(dt / STEP_SIZE) as usize {
            self.tick(STEP_SIZE);
        }
    }

    pub fn tick(&mut self, dt: f32) {
        const G: f32 = 980.0;
        let screen_size = Vec2::new(500.0, 500.0);

        for ball in self.balls.iter_mut() {
            ball.vel.y += G * dt;
            ball.pos += ball.vel * dt;

            let relatative_pos = screen_size / 2.0 - ball.pos;
            if relatative_pos.length_squared() >= (self.outer_radius - self.radius).powi(2) {
                // ! possible energy leak here
                // let central_angle = f32::atan2(relatative_pos.y, relatative_pos.x);
                let reflection_angle = ball.vel.angle_between(relatative_pos);
                let incidence_angle =
                    f32::atan2(ball.vel.y, ball.vel.x) + 2.0 * reflection_angle + PI;
                ball.vel = Vec2::from_angle(incidence_angle) * ball.vel.length();
            }
        }
    }

    pub fn draw(&self) {
        draw_circle_lines(250.0, 250.0, self.outer_radius, 2.0, color::WHITE);

        for ball in self.balls.iter() {
            draw_circle(ball.pos.x, ball.pos.y, self.radius, color::LIME);
        }
    }
}
