use std::fmt;
use std::fs::File;
use std::io::Write;

use super::{Direction, Knot, Tile, MAP_SIZE_HEIGHT, MAP_SIZE_WIDTH, START_COL_IDX, START_ROW_IDX};

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub part: String,
    pub map: Vec<Vec<Tile>>,
    pub knots: Vec<Knot>,
    pub start_idx: (usize, usize),
    pub head_idx: (usize, usize),
    pub tail_idx: (usize, usize),
}

impl Map {
    pub fn new(part: String, num_knots: usize) -> Self {
        assert!(num_knots > 1);

        let mut knots = Vec::<Knot>::new();

        for i in 0..num_knots {
            let is_head = i == 0;
            let is_tail = i == num_knots - 1;

            knots.push(Knot {
                coords: (START_ROW_IDX, START_COL_IDX),
                is_head,
                is_tail,
            });
        }

        let mut map = Map {
            ..Default::default()
        };

        map.part = part;
        map.knots = knots;

        map
    }

    pub fn parse(&mut self, line: String) {
        let mut direction = Direction::Unknown;
        let mut num = 0_usize;

        for el in line.split_whitespace() {
            if el.eq("U") || el.eq("D") || el.eq("L") || el.eq("R") {
                direction = match el {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => Direction::Unknown,
                };
            } else {
                num = el.parse::<usize>().expect("Should be able to parse num.");
            }
        }

        self.travel_head(direction, num);
    }

    fn travel_head(&mut self, direction: Direction, num_steps: usize) {
        if num_steps == 0_usize {
            return;
        }

        for _ in 0..num_steps {
            match direction {
                Direction::Up => self.head_idx.0 -= 1,
                Direction::Down => self.head_idx.0 += 1,
                Direction::Left => self.head_idx.1 -= 1,
                Direction::Right => self.head_idx.1 += 1,
                _ => {}
            }

            self.update_head();

            let mut head_idx = 0_usize;

            for (idx, _) in self.knots.clone().windows(2).enumerate() {
                head_idx = idx;
                self.travel_tail(head_idx);
            }

            self.update_tail(head_idx + 1);
        }
    }

    fn update_head(&mut self) {
        self.map[self.head_idx.0][self.head_idx.1].visited_by_head = true;
        self.knots[0].coords = (self.head_idx.0, self.head_idx.1);
    }

    fn travel_tail(&mut self, head_idx: usize) {
        let tail_idx = head_idx + 1;
        let head = self.knots[head_idx];
        let tail = self.knots[tail_idx];

        let (direction, p, q) = self.get_direction_pq(&head, &tail);

        // Head knot and tail knot are adjacent
        if p + q <= 2 {
            return;
        }

        match direction {
            Direction::Up => {
                self.knots[tail_idx].coords.0 = head.coords.0 + 1;
                self.knots[tail_idx].coords.1 = head.coords.1;
            }
            Direction::Down => {
                self.knots[tail_idx].coords.0 = head.coords.0 - 1;
                self.knots[tail_idx].coords.1 = head.coords.1;
            }
            Direction::Left => {
                self.knots[tail_idx].coords.1 = head.coords.1 + 1;
                self.knots[tail_idx].coords.0 = head.coords.0;
            }
            Direction::Right => {
                self.knots[tail_idx].coords.1 = head.coords.1 - 1;
                self.knots[tail_idx].coords.0 = head.coords.0;
            }
            Direction::Ne => {
                self.knots[tail_idx].coords.0 = head.coords.0 + 1;
                self.knots[tail_idx].coords.1 = head.coords.1 - 1;
            }
            Direction::Se => {
                self.knots[tail_idx].coords.0 = head.coords.0 - 1;
                self.knots[tail_idx].coords.1 = head.coords.1 - 1;
            }
            Direction::Nw => {
                self.knots[tail_idx].coords.0 = head.coords.0 + 1;
                self.knots[tail_idx].coords.1 = head.coords.1 + 1;
            }
            Direction::Sw => {
                self.knots[tail_idx].coords.0 = head.coords.0 - 1;
                self.knots[tail_idx].coords.1 = head.coords.1 + 1;
            }
            _ => {}
        }
    }

    fn update_tail(&mut self, tail_idx: usize) {
        self.map[self.knots[tail_idx].coords.0][self.knots[tail_idx].coords.1].visited_by_tail =
            true;
        self.tail_idx = (self.knots[tail_idx].coords.0, self.knots[tail_idx].coords.1);
    }

    fn get_direction_pq(&self, head: &Knot, tail: &Knot) -> (Direction, i32, i32) {
        let mut direction = Direction::Unknown;

        // Get coordinate differences between the head knot and tail knot
        let p = (head.coords.0 as i32 - tail.coords.0 as i32).pow(2);
        let q = (head.coords.1 as i32 - tail.coords.1 as i32).pow(2);

        if p != q {
            if tail.coords.0 as i32 - head.coords.0 as i32 > 1 {
                direction = Direction::Up;
            } else if head.coords.0 as i32 - tail.coords.0 as i32 > 1 {
                direction = Direction::Down;
            } else if tail.coords.1 as i32 - head.coords.1 as i32 > 1 {
                direction = Direction::Left;
            } else if head.coords.1 as i32 - tail.coords.1 as i32 > 1 {
                direction = Direction::Right;
            }
        } else if head.coords.0 < tail.coords.0 && head.coords.1 < tail.coords.1 {
            direction = Direction::Nw;
        } else if head.coords.0 < tail.coords.0 && head.coords.1 > tail.coords.1 {
            direction = Direction::Ne;
        } else if head.coords.0 > tail.coords.0 && head.coords.1 < tail.coords.1 {
            direction = Direction::Sw;
        } else if head.coords.0 > tail.coords.0 && head.coords.1 > tail.coords.1 {
            direction = Direction::Se;
        }

        (direction, p, q)
    }

    pub fn do_part(&self) {
        let mut head_count = 0_usize;
        let mut tail_count = 0_usize;

        for row in self.map.iter() {
            for tile in row.iter() {
                if tile.visited_by_head {
                    head_count += 1;
                }

                if tile.visited_by_tail {
                    tail_count += 1;
                }
            }
        }

        println!("Part {}:", self.part);
        println!(
            "    The number of positions the Tail has visited is: {}",
            tail_count
        );
        println!(
            "    (Debug) The number of positions the Head has visited is: {}",
            head_count
        );
    }

    pub fn write_map(&self) {
        let filename = format!("day9/Part_{}_output.txt", self.part);
        let mut file = File::create(filename)
            .unwrap_or_else(|filename| panic!("Should be able to open file `{}`.", filename));

        for (row_idx, row) in self.map.iter().enumerate() {
            file.write_all(b"|")
                .expect("Should be able to write beginning of row.");

            for (col_idx, tile) in row.iter().enumerate() {
                if self.start_idx.0 == row_idx && self.start_idx.1 == col_idx {
                    file.write_all(b"S").expect("Should be able to write `S`.");
                } else if self.head_idx.0 == row_idx && self.head_idx.1 == col_idx {
                    file.write_all(b"H").expect("Should be able to write `H`.");
                } else if self.tail_idx.0 == row_idx && self.tail_idx.1 == col_idx {
                    file.write_all(b"T").expect("Should be able to write `T`.");
                } else if tile.visited_by_tail {
                    file.write_all(b"#").expect("Should be able to write `#`.");
                } else if tile.visited_by_head {
                    file.write_all(b"*").expect("Should be able to write `@`.");
                } else {
                    file.write_all(b".").expect("Should be able to write `.`.");
                }
            }

            file.write_all(b"|\n")
                .expect("Should be able to write end of row.");
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        let part = "One".to_string();
        let mut vec = Vec::<Vec<Tile>>::with_capacity(MAP_SIZE_HEIGHT);
        let knots = vec![
            Knot {
                is_head: true,
                is_tail: false,
                coords: (START_ROW_IDX, START_COL_IDX),
            },
            Knot {
                is_head: false,
                is_tail: true,
                coords: (START_ROW_IDX, START_COL_IDX),
            },
        ];
        for _ in 0..MAP_SIZE_HEIGHT {
            vec.push(vec![
                Tile {
                    visited_by_tail: false,
                    visited_by_head: false
                };
                MAP_SIZE_WIDTH
            ]);
        }
        let mut map = Map {
            part,
            map: vec,
            knots,
            start_idx: (START_ROW_IDX, START_COL_IDX),
            head_idx: (START_ROW_IDX, START_COL_IDX),
            tail_idx: (START_ROW_IDX, START_COL_IDX),
        };

        map.map[START_ROW_IDX][START_COL_IDX].visited_by_tail = true;
        map.map[START_ROW_IDX][START_COL_IDX].visited_by_head = true;

        map
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Map:").expect("Should be able to write diagnostic.");

        for (row_idx, row) in self.map.iter().enumerate() {
            write!(f, "|").expect("Should be able to print beginning of row.");
            for (col_idx, tile) in row.iter().enumerate() {
                if self.start_idx.0 == row_idx && self.start_idx.1 == col_idx {
                    write!(f, "S").expect("Should be able to print `S`.");
                } else if self.head_idx.0 == row_idx && self.head_idx.1 == col_idx {
                    write!(f, "H").expect("Should be able to print `H`.");
                } else if self.tail_idx.0 == row_idx && self.tail_idx.1 == col_idx {
                    write!(f, "T").expect("Should be able to print `T`.");
                } else if tile.visited_by_tail {
                    write!(f, "#").expect("Should be able to print `#`.");
                } else if tile.visited_by_head {
                    write!(f, "*").expect("Should be able to print `@`.");
                } else {
                    write!(f, "·").expect("Should be able to print `·`.");
                }
            }
            writeln!(f, "|").expect("Should be able to print end of row.");
        }

        writeln!(f, "Done!").expect("Should be able to print `Done!`.");

        writeln!(f, "Start Index: {:?}", self.start_idx)
            .expect("Should be able to print Start Index.");
        writeln!(f, "Head Index: {:?}", self.head_idx).expect("Should be able to print Head Index");
        writeln!(f, "Tail Index: {:?}", self.tail_idx)
    }
}
