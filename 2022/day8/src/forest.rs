use std::fmt;

use super::Tree;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Forest {
    trees: Vec<Vec<Tree>>,
    num_visible: usize,
}

impl Forest {
    pub fn new() -> Self {
        Forest {
            ..Default::default()
        }
    }

    pub fn parse(&mut self, line: String) {
        let mut row: Vec<Tree> = Vec::new();

        for h in line.chars() {
            let height = h.to_digit(10_u32).unwrap();
            row.push(Tree::new(height as usize));
        }

        self.trees.push(row);
    }

    pub fn check_edge(&mut self, row_idx: usize, col_idx: usize) -> bool {
        if row_idx == 0
            || row_idx == (self.trees.len() - 1)
            || col_idx == 0
            || col_idx == (self.trees[row_idx].len() - 1)
        {
            self.trees[row_idx][col_idx].is_visible = true;
            self.trees[row_idx][col_idx].scenary = 0_usize;
            return true;
        }

        false
    }

    //
    // Part 1
    //

    pub fn outside_top(&mut self, row_idx: usize, col_idx: usize) -> bool {
        if self.trees[row_idx][col_idx].is_visible {
            return true;
        }

        if self.check_edge(row_idx, col_idx) {
            return true;
        }

        let mut is_visible = true;

        for r_idx in (0..row_idx).rev() {
            if self.trees[r_idx][col_idx].value >= self.trees[row_idx][col_idx].value {
                is_visible = false;
                break;
            }
        }

        self.trees[row_idx][col_idx].is_visible = is_visible;

        is_visible
    }

    pub fn outside_right(&mut self, row_idx: usize, col_idx: usize) -> bool {
        if self.trees[row_idx][col_idx].is_visible {
            return true;
        }

        if self.check_edge(row_idx, col_idx) {
            return true;
        }

        let mut is_visible = true;

        for c_idx in (col_idx + 1)..self.trees[row_idx].len() {
            if self.trees[row_idx][c_idx].value >= self.trees[row_idx][col_idx].value {
                is_visible = false;
                break;
            }
        }

        self.trees[row_idx][col_idx].is_visible = is_visible;

        is_visible
    }

    pub fn outside_bottom(&mut self, row_idx: usize, col_idx: usize) -> bool {
        if self.trees[row_idx][col_idx].is_visible {
            return true;
        }

        if self.check_edge(row_idx, col_idx) {
            return true;
        }

        let mut is_visible = true;

        for r_idx in (row_idx + 1)..self.trees.len() {
            if self.trees[r_idx][col_idx].value >= self.trees[row_idx][col_idx].value {
                is_visible = false;
                break;
            }
        }

        self.trees[row_idx][col_idx].is_visible = is_visible;

        is_visible
    }

    pub fn outside_left(&mut self, row_idx: usize, col_idx: usize) -> bool {
        if self.trees[row_idx][col_idx].is_visible {
            return true;
        }

        if self.check_edge(row_idx, col_idx) {
            return true;
        }

        let mut is_visible = true;

        for c_idx in (0..col_idx).rev() {
            if self.trees[row_idx][c_idx].value >= self.trees[row_idx][col_idx].value {
                is_visible = false;
                break;
            }
        }

        self.trees[row_idx][col_idx].is_visible = is_visible;

        is_visible
    }

    pub fn do_part_1(&mut self) {
        let mut num_visible = 0_usize;

        for row_idx in 0..self.trees.len() {
            for col_idx in 0..self.trees[row_idx].len() {
                if self.outside_top(row_idx, col_idx)
                    || self.outside_right(row_idx, col_idx)
                    || self.outside_bottom(row_idx, col_idx)
                    || self.outside_left(row_idx, col_idx)
                {
                    num_visible += 1;
                }
            }
        }

        println!(
            "Part One: The number of trees visible from outside the forest is: {}",
            num_visible
        );
    }

    //
    // Part 2
    //

    pub fn inside_top(&mut self, row_idx: usize, col_idx: usize) -> usize {
        if self.check_edge(row_idx, col_idx) {
            return 0_usize;
        }

        let mut trees = 0_usize;

        for r_idx in (0..row_idx).rev() {
            if self.trees[r_idx][col_idx].value < self.trees[row_idx][col_idx].value {
                trees += 1;
            }
            if self.trees[r_idx][col_idx].value >= self.trees[row_idx][col_idx].value {
                trees += 1;
                break;
            }
        }

        self.trees[row_idx][col_idx].scenary *= trees;

        trees
    }

    pub fn inside_right(&mut self, row_idx: usize, col_idx: usize) -> usize {
        if self.check_edge(row_idx, col_idx) {
            return 0_usize;
        }

        let mut trees = 0_usize;

        for c_idx in (col_idx + 1)..self.trees[row_idx].len() {
            if self.trees[row_idx][c_idx].value < self.trees[row_idx][col_idx].value {
                trees += 1;
            }
            if self.trees[row_idx][c_idx].value >= self.trees[row_idx][col_idx].value {
                trees += 1;
                break;
            }
        }

        self.trees[row_idx][col_idx].scenary *= trees;

        trees
    }

    pub fn inside_bottom(&mut self, row_idx: usize, col_idx: usize) -> usize {
        if self.check_edge(row_idx, col_idx) {
            return 0_usize;
        }

        let mut trees = 0_usize;

        for r_idx in (row_idx + 1)..self.trees.len() {
            if self.trees[r_idx][col_idx].value < self.trees[row_idx][col_idx].value {
                trees += 1;
            }
            if self.trees[r_idx][col_idx].value >= self.trees[row_idx][col_idx].value {
                trees += 1;
                break;
            }
        }

        self.trees[row_idx][col_idx].scenary *= trees;

        trees
    }

    pub fn inside_left(&mut self, row_idx: usize, col_idx: usize) -> usize {
        if self.check_edge(row_idx, col_idx) {
            return 0_usize;
        }

        let mut trees = 0_usize;

        for c_idx in (0..col_idx).rev() {
            if self.trees[row_idx][c_idx].value < self.trees[row_idx][col_idx].value {
                trees += 1;
            }
            if self.trees[row_idx][c_idx].value >= self.trees[row_idx][col_idx].value {
                trees += 1;
                break;
            }
        }

        self.trees[row_idx][col_idx].scenary *= trees;

        trees
    }

    pub fn do_part_2(&mut self) {
        let mut max_scenary = 0_usize;

        for row_idx in 0..self.trees.len() {
            for col_idx in 0..self.trees[row_idx].len() {
                let scenary = self.inside_top(row_idx, col_idx)
                    * self.inside_right(row_idx, col_idx)
                    * self.inside_bottom(row_idx, col_idx)
                    * self.inside_left(row_idx, col_idx);
                if scenary > max_scenary {
                    max_scenary = scenary;
                }
            }
        }

        println!(
            "Part Two: The highest scenic value for any tree is: {}",
            max_scenary
        );

    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Forest:").expect("Should be able to write diagnostic.");

        for row in self.trees.iter() {
            for col in row.iter() {
                write!(f, "{}", col.value).expect("Should be able to write value.");
            }
            writeln!(f).expect("Should be able to write newline.");
        }

        writeln!(f, "Done!")
    }
}
