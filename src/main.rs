use macroquad::prelude::*;

mod player;
mod tile;

use player::*;
use tile::*;

#[macroquad::main(window_conf)]
async fn main() {
    let player_texture = load_texture("res/player/player.png").await.unwrap();
    let tile_texture = load_texture("res/tile/tile-1.png").await.unwrap();

    let start_pos = Vec2::from_array([100f32, 100f32]);
    let mut player = Player::new(start_pos, player_texture);

    let mut tiles: Vec<Tile> = Vec::new();
    for i in 0..5 {
        let pos = Vec2::from_array([100f32 + i as f32 * tile_texture.width(), 200f32]);
        tiles.push(Tile::new(pos, tile_texture, true));
    }
    player.tiles = tiles.clone();

    loop {
        player.update(get_frame_time());

        for tile in tiles.iter_mut() {
            tile.update(get_frame_time());
        }
        player.tiles = tiles.clone();

        clear_background(WHITE);
        player.draw();
        for tile in tiles.iter() {
            tile.draw();
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Platformer".to_owned(),
        window_width: 1920,
        window_height: 1000,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}
