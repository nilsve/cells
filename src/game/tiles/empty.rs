pub mod empty_tile {
    use macroquad::prelude::{BLUE, Color};
    use crate::game::tiles::tile::{Tile, TileAction, TileActions};
    use crate::game::world::World;

    struct AirTileActions;

    impl TileActions for AirTileActions {
        fn get_action(&self, world: &World) -> TileAction {
            TileAction::Nothing
        }

        fn get_color(&self) -> Color {
            BLUE
        }

        fn get_name(&self) -> &'static str {
            "Air"
        }
    }

    pub fn new() -> Tile {
        Tile {
            actions: &AirTileActions,
        }
    }
}


