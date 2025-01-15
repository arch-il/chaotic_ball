use std::f32::consts::PI;

use macroquad::{
    color::{self, Color},
    input,
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines},
};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub struct Simulation {
    pub balls: Vec<Ball>,
    pub radius: f32,
    pub outer_radius: f32,
    pub step_size: f32,
}

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            balls: Vec::new(),
            radius: 10.0,
            outer_radius: 200.0,
            step_size: 0.000001, // MIN
        }
    }

    pub fn update(&mut self, dt: f32) {
        const G: f32 = 980.0;
        let center = Vec2::new(250.0, 250.0);

        self.balls.par_iter_mut().for_each(|ball| {
            for _ in 0..(dt / self.step_size) as usize {
                let dt = self.step_size;

                ball.vel.y += G * dt;
                ball.pos += ball.vel * dt;

                let relative_pos = center - ball.pos;
                if relative_pos.length_squared() >= (self.outer_radius - self.radius).powi(2) {
                    // reflect velocity
                    let reflection_angle = ball.vel.angle_between(relative_pos);
                    let incidence_angle =
                        f32::atan2(ball.vel.y, ball.vel.x) + 2.0 * reflection_angle + PI; // ! incorrect value
                    ball.vel = Vec2::from_angle(incidence_angle) * ball.vel.length();
                    // get ball inside
                    let relative_pos = relative_pos.normalize()
                        * (2.0 * (self.outer_radius - self.radius) - relative_pos.length());
                    ball.pos = center - relative_pos; // ! incorrect value
                }
            }
        });
    }

    pub fn input(&mut self) {
        // spawn balls
        const COLORS: [Color; 9] = [
            color::LIME,
            color::BEIGE,
            color::BLUE,
            color::DARKGREEN,
            color::DARKPURPLE,
            color::GOLD,
            color::GRAY,
            color::GREEN,
            color::LIGHTGRAY,
        ];
        if input::is_mouse_button_pressed(input::MouseButton::Left) {
            let mouse_pos = input::mouse_position();
            let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
            if (Vec2::new(250.0, 250.0) - mouse_pos).length_squared()
                <= (self.outer_radius - self.radius).powi(2)
            {
                self.balls.push(Ball {
                    pos: mouse_pos,
                    vel: Vec2::ZERO,
                    color: COLORS[self.balls.len() % COLORS.len()],
                });
            }
        }

        // change step_size
        if input::is_key_pressed(input::KeyCode::Left) {
            self.step_size *= 10.0;
        }
        if input::is_key_pressed(input::KeyCode::Right) {
            self.step_size /= 10.0;
        }
    }

    pub fn draw(&self) {
        draw_circle_lines(250.0, 250.0, self.outer_radius, 2.0, color::WHITE);

        for ball in self.balls.iter() {
            draw_circle(ball.pos.x, ball.pos.y, self.radius, ball.color);
        }
    }
}
