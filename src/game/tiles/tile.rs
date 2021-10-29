use crate::game::world::World;
use macroquad::prelude::Color;
use std::fmt::{Formatter, Debug};
use core::fmt;
use crate::game::coords::WorldTilePos;

#[derive(Debug)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Self {
        Direction {
            x,
            y,
        }
    }

    pub fn apply_to_world_tile_pos(&self, world_tile_pos: &WorldTilePos) -> WorldTilePos {
        WorldTilePos::new(world_tile_pos.x + self.x, world_tile_pos.y + self.y)
    }
}

// Will swap the positions of the current tile with the target tile
// Otherwise it will replace the previous position with an empty tile
pub struct SwapWithTarget(pub bool);

#[derive(Debug)]
pub enum TileAction {
    Nothing,
    Move(Direction, SwapWithTarget),
}

pub trait Tile {
    fn get_action(&self, world: &World) -> TileAction;
    fn get_color(&self) -> Color;
    fn get_name(&self) -> &'static str;
    fn into_box(&self) -> Box<dyn Tile>;
}

impl Debug for dyn Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series{{{}}}", self.get_name())
    }
}
