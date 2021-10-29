use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Vec2 {
            x,
            y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChunkPos(Vec2);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WorldTilePos(Vec2);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChunkTilePos(Vec2);

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        ChunkPos {
            0: {
                Vec2::new(x, y)
            }
        }
    }
}

impl ChunkTilePos {
    pub fn new(x: i32, y: i32) -> Self {
        ChunkTilePos {
            0: {
                Vec2::new(x, y)
            }
        }
    }
}

impl WorldTilePos {
    pub fn new(x: i32, y: i32) -> Self {
        WorldTilePos {
            0: {
                Vec2::new(x, y)
            }
        }
    }
}

impl Deref for ChunkTilePos {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ChunkTilePos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for ChunkPos {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ChunkPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for WorldTilePos {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for WorldTilePos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
