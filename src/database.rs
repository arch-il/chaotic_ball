use core::f32;

use macroquad::{
    color,
    input::{self, KeyCode},
    shapes::draw_line,
    text::draw_text,
    window,
};

use crate::simulation::Simulation;

pub struct Database {
    kinetic_energy: [f32; 500],
    potential_energy: [f32; 500],
    mechanical_energy: [f32; 500],
    index: usize,
    ball_counter: usize,
    energy_enabed: bool,
}

impl Database {
    pub fn new() -> Self {
        Self {
            kinetic_energy: [0.0; 500],
            potential_energy: [0.0; 500],
            mechanical_energy: [0.0; 500],
            index: 0,
            ball_counter: 0,
            energy_enabed: true,
        }
    }

    pub fn update(&mut self, simulation: &Simulation) {
        self.update_energies(simulation);

        self.ball_counter = simulation.balls.len();

        if input::is_key_pressed(KeyCode::E) {
            self.energy_enabed = !self.energy_enabed;
        }
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

    pub fn draw(&self) {
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

        if self.energy_enabed {
            self.draw_energies();
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
}
