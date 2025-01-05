mod simulation;

use macroquad::{color, time, window};
use simulation::Simulation;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Chaotic Ball".to_owned(),
        window_resizable: false,
        window_width: 500,
        window_height: 500,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();

    loop {
        window::clear_background(color::BLACK);

        simulation.draw();
        simulation.update(time::get_frame_time());

        window::next_frame().await
    }
}
