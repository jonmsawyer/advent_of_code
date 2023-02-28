use std::fmt;

use pathfinding::prelude::astar;

use super::{Pos, Successor};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Map {
    width: isize,
    height: isize,
    pub start_pos: Pos,
    pub end_pos: Pos,
    pub map: Vec<Vec<usize>>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            width: 159,
            height: 41,
            ..Default::default()
        }
    }

    pub fn parse(&mut self, line: String) {
        let mut line_vec = Vec::<usize>::new();

        for c in line.chars() {
            let height = match c {
                'a' => 1_usize,
                'b' => 2_usize,
                'c' => 3_usize,
                'd' => 4_usize,
                'e' => 5_usize,
                'f' => 6_usize,
                'g' => 7_usize,
                'h' => 8_usize,
                'i' => 9_usize,
                'j' => 10_usize,
                'k' => 11_usize,
                'l' => 12_usize,
                'm' => 13_usize,
                'n' => 14_usize,
                'o' => 15_usize,
                'p' => 16_usize,
                'q' => 17_usize,
                'r' => 18_usize,
                's' => 19_usize,
                't' => 20_usize,
                'u' => 21_usize,
                'v' => 22_usize,
                'w' => 23_usize,
                'x' => 24_usize,
                'y' => 25_usize,
                'z' => 26_usize,
                'S' => 27_usize,
                'E' => 28_usize,
                _ => 0_usize,
            };

            line_vec.push(height);
        }

        self.map.push(line_vec);
    }

    pub fn calculate_start_and_end_pos(&mut self) {
        for (row_idx, row) in self.map.iter_mut().enumerate() {
            for (col_idx, col) in row.iter_mut().enumerate() {
                if *col == 27 {
                    self.start_pos = Pos(row_idx as isize, col_idx as isize);
                    *col = 1;
                }

                if *col == 28 {
                    self.end_pos = Pos(row_idx as isize, col_idx as isize);
                    *col = 26;
                }
            }
        }
    }

    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                // Omit diagonal moves (and moving to the same position)
                if (dx + dy).abs() != 1 {
                    continue;
                }

                let new_position = Pos(position.0 + dx, position.1 + dy);

                // If the new position is out of bounds, skip it
                if new_position.0 < 0
                    || new_position.0 >= self.height
                    || new_position.1 < 0
                    || new_position.1 >= self.width
                {
                    continue;
                }

                let new_value = self.map[new_position.0 as usize][new_position.1 as usize];
                let old_value = self.map[position.0 as usize][position.1 as usize];

                // If our next position is no more than 1 height difference, add it to Successors
                if new_value as isize - old_value as isize == 0
                    || new_value as isize - old_value as isize == 1
                {
                    successors.push(Successor { pos: new_position, cost: new_value as isize });
                }

                // If our next position is less than the old position, add it to Successors
                if new_value < old_value {
                    successors.push(Successor { pos: new_position, cost: new_value as isize });
                }
            }
        }

        successors
    }

    pub fn do_part_one(&self) {
        let result = astar(
            &self.start_pos,
            |p| self.get_successors(p).iter().map(|s| (s.pos, s.cost)).collect::<Vec<_>>(),
            |p| (p.0 - self.end_pos.0).abs() + (p.1 - self.end_pos.1).abs(),
            |p| *p == self.end_pos
        );

        println!("Part One:");
        if let Some(result) = result {
            println!("    Shortest path is {}", result.0.len() - 1);
        } else {
            println!("    No path found.");
        }
    }

    pub fn do_part_two(&self) {
        let mut result_vec = Vec::<usize>::new();
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col == 1 {
                    let result = astar(
                        &Pos(row_idx as isize, col_idx as isize),
                        |p| self.get_successors(p).iter().map(|s| (s.pos, s.cost)).collect::<Vec<_>>(),
                        |p| (p.0 - self.end_pos.0).abs() + (p.1 - self.end_pos.1).abs(),
                        |p| *p == self.end_pos
                    );

                    if let Some(result) = result {
                        result_vec.push(result.0.len() - 1);
                    }
                }
            }
        }

        result_vec.sort();

        println!("Part Two:");
        println!("    The shortest path for any given elevation = 1 is {}", result_vec[0]);
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map.iter() {
            for col in row.iter() {
                write!(f, "{}", col).expect("Should be able to write column.");
            }

            writeln!(f).expect("Should be able to write newline.");
        }

        fmt::Result::Ok(())
    }
}
