use macroquad::prelude::*;

///texture_params!(texture, scale)
#[macro_export]
macro_rules! texture_params {
    ($tex: ident, $scale: expr) => {
        DrawTextureParams {
            dest_size: Some($tex.size()*$scale),
            source: Some(Rect::new(0., 0., $tex.width(), $tex.height())),
            ..Default::default()
        }
    };
}

pub struct Player {
    texture: Texture2D,
    pub x: f32,
    pub y: f32,
    lives_left: u8,
    scale: f32
}

impl Player {
    pub fn new(texture: Texture2D, x: f32, y: f32, lives_left: u8, scale: f32) -> Self {
        Self {texture, x, y, lives_left, scale}
    }

    pub fn width(&self) -> f32 {
        self.texture.width()*self.scale
    }

    pub fn height(&self) -> f32 {
        self.texture.height()*self.scale
    }

    pub fn dest_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width(), self.height())
    }

    pub fn update(&mut self, speed: f32) {
        if is_key_down(KeyCode::Right) && (self.x+self.width())<screen_width() {
            self.x += speed;
        }
        if is_key_down(KeyCode::Left) && self.x>0. {
            self.x -= speed;
        }
        if is_key_down(KeyCode::Up) && self.y>0. {
            self.y -= speed;
        }
        if is_key_down(KeyCode::Down) && (self.y+self.height())<screen_height() {
            self.y += speed;
        }
    }

    pub fn draw(&self, tint: Color) {
        let dest = self.texture.size()*self.scale;
        let src = Rect::new(0., 0., self.texture.width(), self.texture.height());

        draw_texture_ex(&self.texture, self.x, self.y, tint, DrawTextureParams {
            dest_size: Some(dest),
            source: Some(src),
            ..Default::default()
        });
    }
}