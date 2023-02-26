use std::fmt;

use super::Operation;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    pub num: usize,
    pub items: Vec<usize>,
    pub operation: Operation,
    pub operation_value: usize,
    pub test_value: usize,
    pub if_true: usize,
    pub if_false: usize,
    pub inspected_items: usize,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey {}: {:?} (Inspected {} items)",
            self.num, self.items, self.inspected_items
        )
    }
}
