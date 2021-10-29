use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::game::chunk::Chunk;
use crate::game::coords::{ChunkPos, WorldTilePos, ChunkTilePos};
use crate::game::tiles::tile::{Tile};

#[derive(Debug)]
pub struct World {
    pub (self) chunk_size: i32,
    pub(crate) chunks: HashMap<i32, HashMap<i32, Arc<Mutex<Chunk>>>>,
}

pub enum WorldError {
    ChunkDoesntExist,
    TileDoenstExist,
}

impl World {
    pub fn new(chunk_size: i32) -> Self {
        World {
            chunk_size,
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk_size(&self) -> i32 {
        self.chunk_size
    }

    pub fn init_world(&mut self, chunks_per_axis: i32) -> Result<(), ()> {
        assert!(chunks_per_axis > 1);

        if !self.chunks.is_empty() {
            return Err(());
        }

        let half = (chunks_per_axis / 2) as i32;
        for x in -half..half {
            self.chunks.insert(x, HashMap::new());
            let x_axis = self.chunks.get_mut(&x).expect("Just inserted x axis, shouldnt be empty");

            for y in -half..half {
                x_axis.insert(y, Arc::new(Mutex::new(Chunk::new(ChunkPos::new(x, y), self.chunk_size))));
            }
        }

        Ok(())
    }

    pub(crate) fn get_chunk(&self, chunk_pos: &ChunkPos) -> Option<Arc<Mutex<Chunk>>> {
        let chunk = self.chunks.get(&chunk_pos.x)?.get(&chunk_pos.y);
        match chunk {
            Some(chunk) => Some(chunk.clone()),
            _ => None
        }
    }

    fn upsert_chunk(&mut self, chunk_pos: &ChunkPos) -> Arc<Mutex<Chunk>> {
        let x_axis;

        if self.chunks.contains_key(&chunk_pos.x) {
            x_axis = self.chunks.get_mut(&chunk_pos.x).unwrap();
        } else {
            self.chunks.insert(chunk_pos.x, HashMap::new());
            x_axis = self.chunks.get_mut(&chunk_pos.x).expect("Just inserted x axis, shouldnt be empty")
        }

        if x_axis.contains_key(&chunk_pos.y) {
            return x_axis.get_mut(&chunk_pos.y).unwrap().clone();
        } else {
            x_axis.insert(chunk_pos.y, Arc::new(Mutex::from(Chunk::new(chunk_pos.to_owned(), self.chunk_size))));
            return x_axis.get_mut(&chunk_pos.y).expect("Just inserted, can't be empty").clone()
        }
    }

    // pub fn get_tile(&self, world_tile_pos: &WorldTilePos) -> Option<&Box<dyn Tile>> {
    //     let (chunk_pos, chunk_tile_pos) = self.extract_world_tile_positions(&world_tile_pos);
    //     let chunk = self.get_chunk(&chunk_pos)?.lock().unwrap();
    //     let tile = chunk.get_tile(&chunk_tile_pos);
    //     Some(tile.())
    // }

    pub fn replace_tile(&mut self, world_tile_pos: &WorldTilePos, tile: &dyn Tile) -> Result<(), WorldError> {
        let (chunk_pos, chunk_tile_pos) = self.extract_world_tile_positions(&world_tile_pos);

        *self.get_chunk(&chunk_pos).ok_or(WorldError::ChunkDoesntExist)?.lock().unwrap().get_tile_mut(&chunk_tile_pos) = tile.into_box();
        // let found_tile = chunk.get_tile_mut(&chunk_tile_pos);
        // *found_tile = tile.into_box();

        Ok(())
    }

    pub fn replace_tile_test(&mut self, world_tile_pos: &WorldTilePos, tile: Box<dyn Tile>) -> Result<(), WorldError> {
        let (chunk_pos, chunk_tile_pos) = self.extract_world_tile_positions(&world_tile_pos);

        let mutex = self.get_chunk(&chunk_pos).ok_or(WorldError::ChunkDoesntExist)?;
        let mut chunk = mutex.lock().unwrap();
        let found_tile = chunk.get_tile_mut(&chunk_tile_pos);
        *found_tile = tile;

        Ok(())
    }

    pub fn upsert_tile(&mut self, world_tile_pos: &WorldTilePos, tile: &Box<dyn Tile>) {
        let (chunk_pos, chunk_tile_pos) = self.extract_world_tile_positions(&world_tile_pos);

        let mutex = self.upsert_chunk(&chunk_pos);
        let mut chunk = mutex.lock().unwrap();
        let found_tile = chunk.get_tile_mut(&chunk_tile_pos);
        // let cloned = dyn_clone::clone_box(&*tile);

        let c = tile.clone();
        let copy: Box<dyn Tile> = c.into_box();
        *found_tile = copy;
    }

    fn convert_to_chunk_pos(&self, coords: &WorldTilePos) -> ChunkPos {
        ChunkPos::new(
            (coords.x as f32 / self.chunk_size as f32).floor() as i32,
            (coords.y as f32 / self.chunk_size as f32).floor() as i32
        )
    }

    pub(crate) fn extract_world_tile_positions(&self, coords: &WorldTilePos) -> (ChunkPos, ChunkTilePos) {
        let chunk_pos = self.convert_to_chunk_pos(coords);
        let chunk_tile_pos = ChunkTilePos::new(
            (chunk_pos.x * self.chunk_size - coords.x).abs(),
            (chunk_pos.y * self.chunk_size - coords.y).abs(),
        );

        (
            chunk_pos,
            chunk_tile_pos
        )
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn extract_world_tile_pos() {
        let world = World::new(10);
        let (chunk_pos, chunk_tile_pos) = world.extract_world_tile_positions(&WorldTilePos::new(9, 9));
        assert_eq!(chunk_pos, ChunkPos::new(0, 0));
        assert_eq!(chunk_tile_pos, ChunkTilePos::new(9, 9));

        let (chunk_pos, chunk_tile_pos) = world.extract_world_tile_positions(&WorldTilePos::new(10, 10));
        assert_eq!(chunk_pos, ChunkPos::new(1, 1));
        assert_eq!(chunk_tile_pos, ChunkTilePos::new(0, 0));


        let (chunk_pos, chunk_tile_pos) = world.extract_world_tile_positions(&WorldTilePos::new(-5, -11));
        assert_eq!(chunk_pos, ChunkPos::new(-1, -2));
        assert_eq!(chunk_tile_pos, ChunkTilePos::new(5, 9));

        let (chunk_pos, chunk_tile_pos) = world.extract_world_tile_positions(&WorldTilePos::new(-20, -101));
        assert_eq!(chunk_pos, ChunkPos::new(-2, -11));
        assert_eq!(chunk_tile_pos, ChunkTilePos::new(0, 9));
    }
}
