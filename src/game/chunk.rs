use crate::game::coords::{ChunkTilePos, ChunkPos, WorldTilePos};
use crate::game::tiles::empty::empty_tile;
use crate::game::world::World;
use crate::game::tiles::tile::{Tile};

#[derive(Debug)]
pub struct Chunk {
    pub chunk_pos: ChunkPos,
    pub chunk_size: i32,
    pub tiles: Vec<Tile>,
}

impl Chunk {
    pub fn new(chunk_pos: ChunkPos, chunk_size: i32) -> Self {
        Chunk {
            chunk_pos,
            chunk_size,
            tiles: Chunk::get_tiles(chunk_size),
        }
    }

    fn get_tiles(chunk_size: i32) -> Vec<Tile> {
        let mut vec: Vec<Tile> = Vec::with_capacity((chunk_size * chunk_size) as usize);
        for i in 0..(chunk_size * chunk_size) {
            vec.push(empty_tile::new());
        }

        vec
    }

    pub fn get_tile(&self, chunk_tile_pos: &ChunkTilePos) -> &Tile {
        assert!(chunk_tile_pos.x * chunk_tile_pos.y < self.chunk_size * self.chunk_size);

        self.tiles.get((chunk_tile_pos.x + (chunk_tile_pos.y * self.chunk_size)) as usize).expect(&*format!("Shouldn't be able to have a non existing tile {} {}", chunk_tile_pos.x, chunk_tile_pos.y))
    }
    pub fn get_tile_mut(&mut self, chunk_tile_pos: &ChunkTilePos) -> &mut Tile {
        assert!(chunk_tile_pos.x * chunk_tile_pos.y < self.chunk_size * self.chunk_size);

        self.tiles.get_mut((chunk_tile_pos.x + (chunk_tile_pos.y * self.chunk_size)) as usize).expect("Shouldn't be able to have a non existing tile")
    }
}
