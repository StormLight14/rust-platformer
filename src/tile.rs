use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Tile {
    pub pos: Vec2,
    pub texture: Texture2D,
    pub size: Vec2,
    pub rect: Rect,
    pub is_solid: bool,
}

impl Tile {
    pub fn new(pos: Vec2, texture: Texture2D, is_solid: bool) -> Self {
        let size: Vec2 = Vec2::from_array([texture.width(), texture.height()]);
        Self {
            pos: pos,
            texture: texture,
            size: size,
            is_solid: is_solid,
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.rect = Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y);
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}
