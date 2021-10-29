use crate::game::tiles::tile::{TileAction, Tile, Direction, SwapWithTarget};
use crate::game::world::World;
use macroquad::color::Color;
use std::fmt::{Debug, Formatter};
use macroquad::prelude::YELLOW;

#[derive(Debug, Copy, Clone)]
pub struct SandTile;

impl Tile for SandTile {
    fn get_action(&self, world: &World) -> TileAction {
        TileAction::Move(Direction::new(0, -1), SwapWithTarget(true))
    }

    fn get_color(&self) -> Color {
        YELLOW
    }

    fn get_name(&self) -> &'static str {
        return "Sand";
    }

    fn into_box(&self) -> Box<dyn Tile> {
        Box::from(*self)
    }
}

impl SandTile {
    pub fn new() -> Self {
        SandTile
    }
}
