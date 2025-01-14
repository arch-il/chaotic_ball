use core::f32;

use macroquad::{
    color::{self, Color},
    input::{self, KeyCode},
    math::Vec2,
    shapes::{draw_line, draw_rectangle_lines},
    text::draw_text,
    time,
};
use ringbuf::{
    traits::{Consumer, Observer, Producer, SplitRef},
    StaticRb,
};

use crate::simulation::Simulation;

const TRAIL_SIZE: usize = 50;
const GRAPH_SIZE: usize = 300;

pub struct Database {
    kinetic_energy: [f32; GRAPH_SIZE],
    potential_energy: [f32; GRAPH_SIZE],
    mechanical_energy: [f32; GRAPH_SIZE],
    frame_time: [f32; GRAPH_SIZE],
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
            kinetic_energy: [0.0; GRAPH_SIZE],
            potential_energy: [0.0; GRAPH_SIZE],
            mechanical_energy: [0.0; GRAPH_SIZE],
            frame_time: [0.0; GRAPH_SIZE],
            index: 0,

            ball_trails: Vec::new(),
            ball_colors: Vec::new(),
            ball_counter: 0,
            step_size: 0.0,

            energy_enabed: true,
            frame_time_enabled: true,
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
        if self.index >= GRAPH_SIZE {
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
        const GAP: f32 = 3.0;

        const ENERGY_RECT: (f32, f32, f32, f32) =
            (500.0 + GAP, 0.0 + GAP, GRAPH_SIZE as f32, 100.0);
        draw_rectangle_lines(
            ENERGY_RECT.0,
            ENERGY_RECT.1,
            ENERGY_RECT.2,
            ENERGY_RECT.3,
            3.0,
            color::PURPLE,
        );

        const FRAME_TIME_RECT: (f32, f32, f32, f32) =
            (500.0 + GAP, 100.0 + 2.0 * GAP, GRAPH_SIZE as f32, 100.0);
        draw_rectangle_lines(
            FRAME_TIME_RECT.0,
            FRAME_TIME_RECT.1,
            FRAME_TIME_RECT.2,
            FRAME_TIME_RECT.3,
            3.0,
            color::LIGHTGRAY,
        );

        let index = if self.index == 0 {
            GRAPH_SIZE - 1
        } else {
            self.index - 1
        };
        let energy_scale = 75.0 / self.mechanical_energy[index];
        let frame_scale = 75.0
            / self
                .frame_time
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

        //? Don't know how to get display refresh rate
        // let fps_line = 1.0 / 165.0 * frame_scale;
        // let fps_line = FRAME_TIME_RECT.1 + FRAME_TIME_RECT.3 - fps_line;
        // draw_line(
        //     FRAME_TIME_RECT.0,
        //     fps_line,
        //     FRAME_TIME_RECT.0 + FRAME_TIME_RECT.2,
        //     fps_line,
        //     3.0,
        //     color::RED,
        // );

        for i in 0..(GRAPH_SIZE - 1) {
            if i + 1 == self.index {
                continue;
            }

            if self.energy_enabed {
                draw_line(
                    ENERGY_RECT.0 + i as f32,
                    ENERGY_RECT.1 + ENERGY_RECT.3 - self.kinetic_energy[i] * energy_scale,
                    ENERGY_RECT.0 + (i + 1) as f32,
                    ENERGY_RECT.1 + ENERGY_RECT.3 - self.kinetic_energy[i + 1] * energy_scale,
                    1.0,
                    color::RED,
                );
                draw_line(
                    ENERGY_RECT.0 + i as f32,
                    ENERGY_RECT.1 + ENERGY_RECT.3 - self.potential_energy[i] * energy_scale,
                    ENERGY_RECT.0 + (i + 1) as f32,
                    ENERGY_RECT.1 + ENERGY_RECT.3 - self.potential_energy[i + 1] * energy_scale,
                    1.0,
                    color::BLUE,
                );
                draw_line(
                    ENERGY_RECT.0 + i as f32,
                    ENERGY_RECT.1 + ENERGY_RECT.3 - self.mechanical_energy[i] * energy_scale,
                    ENERGY_RECT.0 + (i + 1) as f32,
                    ENERGY_RECT.1 + ENERGY_RECT.3 - self.mechanical_energy[i + 1] * energy_scale,
                    1.0,
                    color::PURPLE,
                );
            }

            if self.frame_time_enabled {
                draw_line(
                    FRAME_TIME_RECT.0 + i as f32,
                    FRAME_TIME_RECT.1 + FRAME_TIME_RECT.3 - self.frame_time[i] * frame_scale,
                    FRAME_TIME_RECT.0 + (i + 1) as f32,
                    FRAME_TIME_RECT.1 + FRAME_TIME_RECT.3 - self.frame_time[i + 1] * frame_scale,
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
        let index = if self.index == 0 {
            GRAPH_SIZE - 1
        } else {
            self.index - 1
        };
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
