#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Knot {
    pub coords: (usize, usize),
    pub is_head: bool,
    pub is_tail: bool,
}
