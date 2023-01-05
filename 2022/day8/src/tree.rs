#[derive(Debug, PartialEq, Eq, Default)]
pub struct Tree {
    pub value: usize,
    pub is_visible: bool,
    pub scenary: usize,
}

impl Tree {
    pub fn new(value: usize) -> Self {
        Tree {
            value,
            is_visible: false,
            scenary: 1_usize,
        }
    }
}
