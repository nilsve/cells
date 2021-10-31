use crate::game::world::World;
use macroquad::prelude::Color;
use std::fmt::{Formatter, Debug, Pointer};
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
#[derive(Debug)]
pub struct SwapWithTarget(pub bool);

#[derive(Debug)]
pub enum TileAction {
    Nothing,
    Move(Direction, SwapWithTarget),
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub(crate) actions: &'static dyn TileActions,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.actions.get_name())
    }
}

pub trait TileActions: 'static {
    fn get_action(&self, world: &World) -> TileAction;
    fn get_color(&self) -> Color;
    fn get_name(&self) -> &'static str;
}
