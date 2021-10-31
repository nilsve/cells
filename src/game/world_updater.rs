use crate::game::world::{World, WorldError};
use crate::game::chunk::Chunk;
use crate::game::coords::{ChunkPos, ChunkTilePos, WorldTilePos};
use crate::game::tiles::empty::empty_tile;
use crate::game::tiles::tile::{Tile, TileAction};

impl World {
    pub fn update(&mut self) {
        self.get_updatable_chunks(&|chunk_pos| {
            let mutex = self.get_chunk(&chunk_pos).expect("Cant be empty");
            let mut chunk = mutex.lock().unwrap();

            for tile_x in 0..chunk.chunk_size {
                for tile_y in 0..chunk.chunk_size {
                    let world_tile_pos = WorldTilePos::new(chunk_pos.x * self.get_chunk_size() + tile_x,chunk_pos.y * self.get_chunk_size() + tile_y);
                    let (_, chunk_tile_pos) = self.extract_world_tile_positions(&world_tile_pos);
                    let tile = chunk.get_tile(&chunk_tile_pos).clone();
                    let action = tile.actions.get_action(&self);
                    if let TileAction::Move(direction, swap_with_target) = action {
                        let (target_chunk_pos, target_chunk_tile_pos) = self.extract_world_tile_positions(&WorldTilePos::new(world_tile_pos.x + direction.x, world_tile_pos.y + direction.y));

                        if target_chunk_pos.x != chunk_pos.x || target_chunk_pos.y != chunk_pos.y {
                            if let Some(mutex) = self.get_chunk(&target_chunk_pos) {
                                let mut target_chunk = mutex.lock().unwrap();
                                let target_tile = target_chunk.get_tile_mut(&target_chunk_tile_pos);

                                if !swap_with_target.0 {
                                    *target_tile = tile.clone();
                                    *chunk.get_tile_mut(&chunk_tile_pos) = empty_tile::new();
                                } else {
                                    let copy = target_tile.clone();
                                    *target_tile = tile.clone();
                                    *chunk.get_tile_mut(&chunk_tile_pos) = copy;
                                }
                            }
                        } else {
                            let target_tile = chunk.get_tile_mut(&target_chunk_tile_pos);

                                if !swap_with_target.0 {
                                *target_tile = tile.clone();
                                *chunk.get_tile_mut(&chunk_tile_pos) = empty_tile::new();
                            } else {
                                let copy = target_tile.clone();
                                *target_tile = tile.clone();
                                *chunk.get_tile_mut(&chunk_tile_pos) = copy;
                            }
                        }
                    }
                }
            }
        });
    }

    fn get_updatable_chunks(&self, callback: &dyn Fn(ChunkPos)) {
        let chunk_x_coords: Vec<i32> = self.chunks.keys().map(|x| *x).collect();
        chunk_x_coords.iter().for_each(|chunk_x| {
            let chunk_y_coords: Vec<i32> = self.chunks.keys().map(|y| *y).collect();
            chunk_y_coords.iter().for_each(|chunk_y| {
                callback(ChunkPos::new(*chunk_x, *chunk_y));
            });
        });
    }
}
