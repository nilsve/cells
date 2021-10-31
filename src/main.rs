use crate::game::coords::WorldTilePos;
use std::thread::sleep;
use std::time::Duration;
use macroquad::prelude::*;
use crate::game::tiles::sand::sand_tile;

mod game;

#[macroquad::main("Cells")]
async fn main() {
    let mut world = game::world::World::new(10);
    world.init_world(4);
    world.replace_tile(&WorldTilePos::new(-20, 0), sand_tile::new());
    // world.replace_tile(&WorldTilePos::new(3, 2), &SandTile::new());

    let mut offset = (0., 0.);
    loop {
        if is_key_down(KeyCode::Left) {
            offset.0 -= 0.1;
        }
        if is_key_down(KeyCode::Right) {
            offset.0 += 0.1;
        }
        if is_key_down(KeyCode::Up) {
            offset.1 += 0.1;
        }
        if is_key_down(KeyCode::Down) {
            offset.1 -= 0.1;
        }

        set_camera(&Camera2D {
            target: vec2(0.0, 0.0),
            rotation: 0.0,
            zoom: vec2(0.01, 0.01 * screen_width() / screen_height()),
            offset: vec2(offset.0, offset.1),
            ..Default::default()
        });

        world.render();

        world.update();

        set_default_camera();
        next_frame().await;

        sleep(Duration::from_millis(100));
    }
}
