use macroquad::prelude::*;

mod audio;
use audio::*;

#[macroquad::main(window_conf)]
async fn main() {
    let audio = AudioDevice::new();
    let music = Music::load(&audio, "assets/sample.qoa");
    music.play();

    loop {
        music.update();

        clear_background(BLACK);
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "audio".to_string(),
        window_width: 800,
        window_height: 450,
        icon: None,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}