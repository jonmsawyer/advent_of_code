#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Tile {
    pub visited_by_tail: bool,
    pub visited_by_head: bool,
}
