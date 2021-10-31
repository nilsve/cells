use crate::game::world::World;
use macroquad::prelude::*;
use crate::game::coords::{WorldTilePos, ChunkTilePos};

const TILE_SIZE: f32 = 5.0;

impl World {
    pub fn render(&self) {
        clear_background(RED);

        self.chunks.keys().for_each(|chunk_x| {
            let x_axis = self.chunks.get(chunk_x).unwrap();
            self.chunks.keys().for_each(|chunk_y| {
                let chunk = x_axis.get(chunk_y).unwrap().lock().unwrap();

                for tile_x in 0..chunk.chunk_size {
                    for tile_y in 0..chunk.chunk_size {
                        let chunk_tile_pos = ChunkTilePos::new(tile_x, tile_y);
                        let world_tile_pos = WorldTilePos::new(
                            chunk_x * self.get_chunk_size() + chunk_tile_pos.x,
                            chunk_y * self.get_chunk_size() + chunk_tile_pos.y,
                        );

                        let tile = chunk.get_tile(&chunk_tile_pos);

                        draw_rectangle(world_tile_pos.x as f32 * TILE_SIZE, world_tile_pos.y as f32 * TILE_SIZE, TILE_SIZE, TILE_SIZE, tile.actions.get_color());
                    }
                }

                draw_rectangle_lines(*chunk_x as f32 * TILE_SIZE * self.get_chunk_size() as f32, *chunk_y as f32 * TILE_SIZE * self.get_chunk_size() as f32, self.get_chunk_size() as f32 * TILE_SIZE as f32, self.get_chunk_size() as f32 * TILE_SIZE as f32, 1.0, GRAY);
            });
        });
    }
}


