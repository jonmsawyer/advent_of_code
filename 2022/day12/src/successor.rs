use super::Pos;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: isize,
}

// Used to make writing tests easier
impl PartialEq<(Pos, usize)> for Successor {
    fn eq(&self, other: &(Pos, usize)) -> bool {
        self.pos == other.0 && self.cost == other.1 as isize
    }
}
