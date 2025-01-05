use macroquad::{
    color,
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines},
    window::{screen_height, screen_width},
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
                pos: Vec2::new(250.0, 250.0),
                vel: Vec2::ZERO,
            }],
            radius: 10.0,
            outer_radius: 200.0,
        }
    }

    pub fn draw(&self) {
        draw_circle_lines(
            screen_width() / 2.0,
            screen_height() / 2.0,
            self.outer_radius,
            2.0,
            color::WHITE,
        );

        for ball in self.balls.iter() {
            draw_circle(ball.pos.x, ball.pos.y, self.radius, color::LIME);
        }
    }
}
