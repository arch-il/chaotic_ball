mod database;
mod simulation;

use database::Database;
use macroquad::{color, input, time, window};
use simulation::Simulation;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Chaotic Ball".to_owned(),
        window_resizable: false,
        window_width: 500,
        window_height: 550,
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

        simulation.update(time::get_frame_time());
        database.update(&simulation);

        if input::is_key_down(input::KeyCode::Escape) {
            break;
        }

        window::next_frame().await
    }
}
