use std::fmt;

use super::{Monkey, Operation};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Monkeys(pub Vec<Monkey>);

impl Monkeys {
    pub fn do_part_one(&mut self) {
        println!("Part One:");

        let rounds = 20_usize;

        for _ in 0..rounds {
            for idx in 0..self.0.len() {
                for _ in 0..self.0[idx].items.len() {
                    let mut item = self.0[idx].items.remove(0);

                    // This monkey has inspected this item
                    self.0[idx].inspected_items += 1;

                    // Multiply or add the worry level
                    match self.0[idx].operation {
                        Operation::Multiply => item *= self.0[idx].operation_value,
                        Operation::Add => item += self.0[idx].operation_value,
                        Operation::Pow => item = item.pow(2),
                        Operation::Unknown => {}
                    }

                    // Divide worry level by 3
                    item /= 3;

                    // Toss the item to the next monkey
                    if item % self.0[idx].test_value == 0 {
                        let true_idx = self.0[idx].if_true;
                        self.0[true_idx].items.push(item);
                    } else {
                        let false_idx = self.0[idx].if_false;
                        self.0[false_idx].items.push(item);
                    }
                }
            }
        }

        println!("    Monkey business: {}", self.top_two())
    }

    pub fn do_part_two(&mut self) {
        println!("Part Two:");

        let rounds = 10_000_usize;

        // Least Common Multiple
        // Thanks to Travis Veazey <https://github.com/Kromey>
        let mut lcm = 1_usize;

        // We calculate the least common multiple here, because on a 64-bit architecture we get
        // integer overflow when multiplying the individual items. We reduce each multiple by the
        // "LCM" to keep integer overflow nonexistent, since each test value in each Monkey's
        // "Test" input is a prime number.
        for monkey in self.0.iter() {
            lcm *= monkey.test_value;
        }

        for _ in 0..rounds {
            for idx in 0..self.0.len() {
                for _ in 0..self.0[idx].items.len() {
                    let mut item = self.0[idx].items.remove(0);

                    // This monkey has inspected this item
                    self.0[idx].inspected_items += 1;

                    // Multiply or add the worry level
                    match self.0[idx].operation {
                        Operation::Multiply => item *= self.0[idx].operation_value,
                        Operation::Add => item += self.0[idx].operation_value,
                        Operation::Pow => item = item.pow(2),
                        Operation::Unknown => {}
                    }

                    // Toss the item to the next monkey
                    if item % self.0[idx].test_value == 0 {
                        let true_idx = self.0[idx].if_true;
                        // LCM applied here
                        self.0[true_idx].items.push(item % lcm);
                    } else {
                        let false_idx = self.0[idx].if_false;
                        // LCM applied here.
                        self.0[false_idx].items.push(item % lcm);
                    }
                }
            }
        }

        println!("    Monkey business: {}", self.top_two())
    }

    pub fn top_two(&self) -> usize {
        let mut top_1 = 0_usize;
        let mut top_2 = 0_usize;

        for monkey in &self.0 {
            if monkey.inspected_items > top_1 {
                top_2 = top_1;
                top_1 = monkey.inspected_items;
            } else if monkey.inspected_items > top_2 {
                top_2 = monkey.inspected_items;
            }
        }

        top_1 * top_2
    }
}

impl Monkeys {
    pub fn new() -> Self {
        Monkeys {
            ..Default::default()
        }
    }
}

impl fmt::Display for Monkeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, monkey) in self.0.iter().enumerate() {
            writeln!(f, "> Monkey {}:", idx).expect("Should be able to write Monkey Number.");
            writeln!(f, ">  Starting items: {:?}", monkey.items)
                .expect("Should be able to write Starting Items.");
            let (op, op_val) = match monkey.operation {
                Operation::Multiply => ("*".to_string(), format!("{}", monkey.operation_value)),
                Operation::Add => ("+".to_string(), format!("{}", monkey.operation_value)),
                Operation::Pow => ("*".to_string(), "old".to_string()),
                Operation::Unknown => ("Unknown".to_string(), "Unknown".to_string()),
            };
            writeln!(f, ">  Operation: new = old {} {}", op, op_val)
                .expect("Should be able to write Operation.");
            writeln!(f, ">  Test: divisible by {}", monkey.test_value)
                .expect("Should be able to write Test.");
            writeln!(f, ">    If true: throw to monkey {}", monkey.if_true)
                .expect("Should be able to write If True.");
            writeln!(f, ">    If false: throw to monkey {}", monkey.if_false)
                .expect("Should be able to write If False.");
            writeln!(f, ">").expect("Should be able to write newline.");
        }

        fmt::Result::Ok(())
    }
}
