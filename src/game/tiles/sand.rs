pub mod sand_tile {
    use macroquad::prelude::{BLUE, Color, YELLOW};
    use crate::game::tiles::tile::{Direction, SwapWithTarget, Tile, TileAction, TileActions};
    use crate::game::world::World;

    struct SandTileActions;

    impl TileActions for SandTileActions {
        fn get_action(&self, world: &World) -> TileAction {
            TileAction::Move(Direction::new(0, -1), SwapWithTarget(true))
        }

        fn get_color(&self) -> Color {
            YELLOW
        }

        fn get_name(&self) -> &'static str {
            "Air"
        }
    }

    pub fn new() -> Tile {
        Tile {
            actions: &SandTileActions,
        }
    }
}
