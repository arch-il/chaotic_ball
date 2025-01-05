use macroquad::{
    color,
    shapes::draw_circle_lines,
    window::{self, screen_height, screen_width},
};

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
    loop {
        window::clear_background(color::BLACK);

        draw_circle_lines(
            screen_width() / 2.0,
            screen_height() / 2.0,
            screen_width() * 0.8 / 2.0,
            3.0,
            color::WHITE,
        );

        window::next_frame().await
    }
}
