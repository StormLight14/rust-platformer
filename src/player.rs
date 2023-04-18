use crate::tile::*;
use macroquad::prelude::*;
use rust_platformer::col::*;

const PLAYER_JUMP_STRENGTH: f32 = 200f32;
const MAX_GRAVITY: f32 = 700f32;
const GRAVITY_ACCELERATION: f32 = 5f32;
const PLAYER_SPEED: f32 = 500f32;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct Colliding {
    pub x: bool,
    pub y: bool,
}

#[derive(Debug)]
pub struct Player {
    pub velocity: Vec2,
    pub speed: f32,
    pub pos: Vec2,
    pub texture: Texture2D,
    pub size: Vec2,
    pub rect: Rect,
    pub gravity_speed: f32,
    pub on_ground: bool,
    pub jump_amount: u8,
    pub can_jump: bool,
    pub tiles: Vec<Tile>,
    pub colliding: Colliding,
}

impl Player {
    pub fn new(pos: Vec2, texture: Texture2D) -> Self {
        let size = Vec2::from_array([texture.width(), texture.height()]);
        Self {
            // movement
            velocity: Vec2::from_array([0f32, 0f32]),
            speed: PLAYER_SPEED,
            pos: pos,

            // display
            texture: texture,

            // collision
            size: size,
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
            colliding: Colliding { x: false, y: false },

            // gravity
            gravity_speed: 0f32,
            on_ground: false,
            jump_amount: 0,
            can_jump: true,
            tiles: Vec::new(),
        }
    }

    pub fn input(&mut self, delta: f32) {
        if is_key_pressed(KeyCode::Space) && self.can_jump == true {
            self.gravity_speed = -PLAYER_JUMP_STRENGTH;
            self.jump_amount += 1;
        }
        if is_key_pressed(KeyCode::R) {
            // reset for debugging
            self.pos.y = 0f32;
            self.gravity_speed = 0f32;
        }
        if is_key_down(KeyCode::A) {
            self.velocity.x -= self.speed;
        }

        if is_key_down(KeyCode::D) {
            self.velocity.x += self.speed;
        }

        if self.jump_amount >= 200 {
            self.can_jump = false;
        }
    }

    pub fn move_self(&mut self, delta: f32) {
        self.pos.x += self.velocity.x * delta;
        self.collision(Direction::Horizontal);
        self.pos.y += self.velocity.y * delta;
        self.collision(Direction::Vertical);

        self.velocity.x = 0f32;

        if self.colliding.y {
            self.velocity.y = 0f32;
        }
    }

    pub fn collision(&mut self, direction: Direction) {
        if direction == Direction::Horizontal {
            for tile in self.tiles.iter() {
                if self.rect.overlaps(&tile.rect) {
                    // moving left
                    if self.velocity.x < 0f32 {
                        self.colliding.x = true;
                        self.pos.x = tile.rect.right()
                    }
                    // moving right
                    if self.velocity.x > 0f32 {
                        self.colliding.x = true;
                        self.pos.x = tile.rect.left() - self.size.x;
                    }
                }
            }
        }
        if direction == Direction::Vertical {
            for tile in self.tiles.iter() {
                if self.rect.overlaps(&tile.rect) {
                    println!("{:?} {:?}", self.pos, tile.pos);
                    // moving down
                    if self.velocity.y > 0f32 {
                        self.colliding.y = true;
                        self.gravity_speed = 0f32;
                        self.jump_amount = 0;
                        self.pos.y = tile.rect.top() - self.size.y;
                    }
                    // moving up
                    if self.velocity.y < 0f32 {
                        self.colliding.y = true;
                        self.gravity_speed = 0f32;
                        self.pos.y = tile.rect.bottom();
                    }
                } 
                
                    else if self.velocity.y != 0f32 {
                        self.colliding.y = false;
                    }}
            }
        }
    }

    pub fn gravity(&mut self, delta: f32) {
        if self.colliding.y == false {
            if self.gravity_speed < MAX_GRAVITY {
                self.gravity_speed += GRAVITY_ACCELERATION;
            }
            self.velocity.y = self.gravity_speed;
        }

        for tile in self.tiles.iter() {
            if self.rect.overlaps(&tile.rect) == false {
                self.colliding.y = false;
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.input(delta);
        self.move_self(delta);
        self.gravity(delta);

        /*
        println!(
            "Velocity: {:?}, Gravity: {:?}, Pos: {:?}",
            self.velocity, self.gravity_speed, self.pos
        );
        */
        self.rect = Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y);
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}
