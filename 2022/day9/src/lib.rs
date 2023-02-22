const MAP_SIZE_HEIGHT: usize = 233;
const MAP_SIZE_WIDTH: usize = 354;
const START_ROW_IDX: usize = 218;
const START_COL_IDX: usize = 18;

pub mod map;
pub use map::Map;

pub mod direction;
pub use direction::Direction;

mod tile;
pub use tile::Tile;

mod knot;
pub use knot::Knot;
