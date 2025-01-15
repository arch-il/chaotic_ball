mod database;
mod simulation;

use std::time::Instant;

use database::Database;
use macroquad::{color, input, time, window};
use simulation::Simulation;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Chaotic Ball".to_owned(),
        window_resizable: false,
        window_width: 806,
        window_height: 500,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();
    let mut database = Database::new();

    loop {
        window::clear_background(color::BLACK);

        database.draw();
        simulation.draw();

        simulation.input();
        database.input();

        let start = Instant::now();
        simulation.update(time::get_frame_time());
        let simulation_time = (Instant::now() - start).as_secs_f32();

        database.update(&simulation, simulation_time);

        if input::is_key_down(input::KeyCode::Escape) {
            break;
        }

        window::next_frame().await
    }
}
