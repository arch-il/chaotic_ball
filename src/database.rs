use core::f32;

use macroquad::{
    color::{self, Color},
    input::{self, KeyCode},
    math::Vec2,
    shapes::draw_line,
    text::draw_text,
    time, window,
};
use ringbuf::{
    traits::{Consumer, Observer, Producer, SplitRef},
    StaticRb,
};

use crate::simulation::Simulation;

const TRAIL_SIZE: usize = 50;

pub struct Database {
    kinetic_energy: [f32; 500],
    potential_energy: [f32; 500],
    mechanical_energy: [f32; 500],
    frame_time: [f32; 500],
    index: usize,

    ball_trails: Vec<StaticRb<Vec2, TRAIL_SIZE>>,
    ball_colors: Vec<Color>,
    ball_counter: usize,
    step_size: f32,

    energy_enabed: bool,
    frame_time_enabled: bool,
    trial_enabled: bool,
    info_enabled: bool,
}

impl Database {
    pub fn new() -> Self {
        Self {
            kinetic_energy: [0.0; 500],
            potential_energy: [0.0; 500],
            mechanical_energy: [0.0; 500],
            frame_time: [0.0; 500],
            index: 0,

            ball_trails: Vec::new(),
            ball_colors: Vec::new(),
            ball_counter: 0,
            step_size: 0.0,

            energy_enabed: true,
            frame_time_enabled: false,
            trial_enabled: true,
            info_enabled: true,
        }
    }

    pub fn update(&mut self, simulation: &Simulation) {
        self.update_graphs(simulation);
        self.update_ball_trails(simulation);

        self.ball_counter = simulation.balls.len();
        self.step_size = simulation.step_size;
    }

    fn update_graphs(&mut self, simulation: &Simulation) {
        self.kinetic_energy[self.index] = simulation
            .balls
            .iter()
            .map(|ball| ball.vel)
            .fold(0.0, |acc, v| acc + (v.length() / 100.0).powi(2))
            / 2.0;

        self.potential_energy[self.index] = simulation
            .balls
            .iter()
            .map(|ball| ball.pos)
            .fold(0.0, |acc, c| acc + (450.0 - c.y) / 100.0 * 9.8);

        self.mechanical_energy[self.index] =
            self.kinetic_energy[self.index] + self.potential_energy[self.index];

        self.frame_time[self.index] = time::get_frame_time();

        self.index += 1;
        if self.index >= window::screen_width() as usize {
            self.index = 0;
        }
    }

    fn update_ball_trails(&mut self, simulation: &Simulation) {
        if simulation.balls.len() > self.ball_trails.len() {
            self.ball_trails
                .push(StaticRb::<Vec2, TRAIL_SIZE>::default());
            self.ball_colors
                .push(simulation.balls.last().unwrap().color);
        }

        for (trail, ball) in self.ball_trails.iter_mut().zip(simulation.balls.iter()) {
            let (mut prod, mut cons) = trail.split_ref();
            if cons.is_full() {
                cons.try_pop();
            }
            let _ = prod.try_push(ball.pos);
        }
    }

    pub fn input(&mut self) {
        if input::is_key_pressed(KeyCode::E) {
            self.energy_enabed = !self.energy_enabed;
        }
        if input::is_key_pressed(KeyCode::F) {
            self.frame_time_enabled = !self.frame_time_enabled;
        }
        if input::is_key_pressed(KeyCode::T) {
            self.trial_enabled = !self.trial_enabled;
        }
        if input::is_key_pressed(KeyCode::I) {
            self.info_enabled = !self.info_enabled;
        }
    }

    pub fn draw(&self) {
        if self.energy_enabed || self.frame_time_enabled {
            self.draw_graphs();
        }
        if self.trial_enabled {
            self.draw_trails();
        }
        if self.info_enabled {
            self.draw_info();
        }
    }

    fn draw_graphs(&self) {
        let index = if self.index == 0 { 499 } else { self.index - 1 };
        let energy_scale = 75.0 / self.mechanical_energy[index];
        let frame_scale = self.frame_time.iter().filter(|&&x| x != 0.0);
        let frame_scale = 40.0 / (frame_scale.clone().sum::<f32>() / frame_scale.count() as f32);

        for i in 0..499 {
            if i + 1 == self.index {
                continue;
            }

            if self.energy_enabed {
                draw_line(
                    i as f32,
                    window::screen_height() - self.kinetic_energy[i] * energy_scale,
                    (i + 1) as f32,
                    window::screen_height() - self.kinetic_energy[i + 1] * energy_scale,
                    1.0,
                    color::RED,
                );
                draw_line(
                    i as f32,
                    window::screen_height() - self.potential_energy[i] * energy_scale,
                    (i + 1) as f32,
                    window::screen_height() - self.potential_energy[i + 1] * energy_scale,
                    1.0,
                    color::BLUE,
                );
                draw_line(
                    i as f32,
                    window::screen_height() - self.mechanical_energy[i] * energy_scale,
                    (i + 1) as f32,
                    window::screen_height() - self.mechanical_energy[i + 1] * energy_scale,
                    1.0,
                    color::PURPLE,
                );
            }

            if self.frame_time_enabled {
                draw_line(
                    i as f32,
                    window::screen_height() - self.frame_time[i] * frame_scale,
                    (i + 1) as f32,
                    window::screen_height() - self.frame_time[i + 1] * frame_scale,
                    1.0,
                    color::LIGHTGRAY,
                );
            }
        }
    }

    fn draw_trails(&self) {
        for (ball_id, trail) in self.ball_trails.iter().enumerate() {
            let count = trail.iter().count();
            let mut iter = trail.iter().enumerate().peekable();
            while let Some((i, curr)) = iter.next() {
                if let Some((_, next)) = iter.peek() {
                    let mut color = self.ball_colors[ball_id];
                    color.a = i as f32 / count as f32;
                    draw_line(curr.x, curr.y, next.x, next.y, 1.0, color);
                }
            }
        }
    }

    fn draw_info(&self) {
        let index = if self.index == 0 { 499 } else { self.index - 1 };
        draw_text(
            &format!(
                "balls: {}; step_size: {}; energy: {}",
                self.ball_counter,
                self.step_size,
                self.kinetic_energy[index] + self.potential_energy[index],
            ),
            5.0,
            12.0,
            20.0,
            color::LIGHTGRAY,
        );
    }
}
