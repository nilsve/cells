use crate::game::tiles::tile::{TileAction, Tile};
use crate::game::world::World;
use macroquad::color::Color;
use macroquad::prelude::BLUE;

#[derive(Debug, Copy, Clone)]
pub struct EmptyTile;

impl Tile for EmptyTile {
    fn get_action(&self, _: &World) -> TileAction {
        TileAction::Nothing
    }

    fn get_color(&self) -> Color {
        BLUE
    }

    fn get_name(&self) -> &'static str {
        return "Air";
    }

    fn into_box(&self) -> Box<dyn Tile> {
        let result: Box<dyn Tile> = Box::from(self.clone());
        result
    }
}

impl EmptyTile {
    pub fn new() -> Self {
        EmptyTile
    }
}
