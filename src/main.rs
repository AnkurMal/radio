use macroquad::prelude::*;

use player::Player;
mod player;
mod sound;
use sound::*;


///text_params!(font, size, color)
macro_rules! text_params {
    ($font: ident, $size: literal, $color: expr) => {
        TextParams {
            font: Some(&$font),
            font_size: $size,
            color: $color,
            ..Default::default()
        }
    };
}

#[derive(Copy, Clone, Debug)]
struct Projectile {
    x: f32,
    y: f32
}

impl Projectile {
    const SCALE: f32 = 0.6;

    fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let audio = init_audio_device();
    let screen_width = screen_width();
    let _screen_height = screen_height();
    let mut player_proj = Vec::new();

    set_pc_assets_folder("assets");

    let sound = load_sound("assets/sound/gun.mp3");
    let font = load_ttf_font("font/edge.ttf").await.unwrap();
    let laser = load_texture("images/png/laserGreen.png").await.unwrap();
    let mut player = Player::new(load_texture("images/png/player.png").await.unwrap(),
                                 200., 200., 3, 0.5);

    loop {
        let mut i = 0;

        player.update(5.);
        if is_key_pressed(KeyCode::Space) {
            player_proj.push(Projectile::new(player.x+player.width()/2., player.y));
            audio.play(&sound);
        }

        clear_background(BLACK);

        while i<player_proj.len() {
            draw_texture_ex(&laser, player_proj[i].x, player_proj[i].y, WHITE, texture_params!(laser, Projectile::SCALE));
            player_proj[i].y -= 9.;
            if player_proj[i].y+laser.height()*Projectile::SCALE<0. {
                player_proj.remove(i);
            }
            else {
                i += 1;
            }
        }

        player.draw(BLUE);
        draw_text_ex("Score: 0", screen_width-120., 30., text_params!(font, 40, WHITE));

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "sample".to_string(),
        window_width: 700,
        window_height: 900,
        icon: None,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}