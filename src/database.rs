use core::f32;

use macroquad::{
    color::{self, Color},
    input::{self, KeyCode},
    math::Vec2,
    shapes::draw_line,
    text::draw_text,
    window,
};
use ringbuf::{
    traits::{Consumer, Observer, Producer, SplitRef},
    StaticRb,
};

use crate::simulation::Simulation;

const TRAIL_SIZE: usize = 250;

pub struct Database {
    kinetic_energy: [f32; 500],
    potential_energy: [f32; 500],
    mechanical_energy: [f32; 500],
    ball_trails: Vec<StaticRb<Vec2, TRAIL_SIZE>>,
    ball_colors: Vec<Color>,
    ball_counter: usize,
    index: usize,

    energy_enabed: bool,
    trial_enabled: bool,
    info_enabled: bool,
}

impl Database {
    pub fn new() -> Self {
        Self {
            kinetic_energy: [0.0; 500],
            potential_energy: [0.0; 500],
            mechanical_energy: [0.0; 500],
            ball_trails: Vec::new(),
            ball_colors: Vec::new(),
            ball_counter: 0,
            index: 0,

            energy_enabed: true,
            trial_enabled: true,
            info_enabled: true,
        }
    }

    pub fn update(&mut self, simulation: &Simulation) {
        self.update_energies(simulation);
        self.update_ball_trails(simulation);

        self.ball_counter = simulation.balls.len();
    }

    fn update_energies(&mut self, simulation: &Simulation) {
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
        if input::is_key_pressed(KeyCode::T) {
            self.trial_enabled = !self.trial_enabled;
        }
        if input::is_key_pressed(KeyCode::I) {
            self.info_enabled = !self.info_enabled;
        }
    }

    pub fn draw(&self) {
        if self.energy_enabed {
            self.draw_energies();
        }
        if self.trial_enabled {
            self.draw_trails();
        }
        if self.info_enabled {
            self.draw_info();
        }
    }

    fn draw_energies(&self) {
        let scale: f32 =
            75.0 / self.mechanical_energy[if self.index == 0 { 499 } else { self.index - 1 }];

        let mut energies = self
            .kinetic_energy
            .iter()
            .zip(self.potential_energy.iter())
            .zip(self.mechanical_energy.iter())
            .rev()
            .enumerate()
            .peekable();

        while let Some((curr_i, ((&curr_k, &curr_p), &curr_m))) = energies.next() {
            if let Some((next_i, ((next_k, next_p), next_m))) = energies.peek() {
                if *next_i == 500 - self.index || curr_i == 500 - self.index {
                    continue;
                }

                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_k * scale,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_k * scale,
                    1.0,
                    color::RED,
                );
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_p * scale,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_p * scale,
                    1.0,
                    color::BLUE,
                );
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_m * scale,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_m * scale,
                    1.0,
                    color::PURPLE,
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
        draw_text(
            &format!(
                "balls: {}; energy: {}",
                self.ball_counter,
                self.mechanical_energy[if self.index == 0 { 499 } else { self.index - 1 }],
            ),
            5.0,
            12.0,
            20.0,
            color::LIGHTGRAY,
        );
    }
}
