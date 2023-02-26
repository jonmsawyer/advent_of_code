use utils::*;

use day11::{Monkey, Monkeys, Operation};

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut line_no = 1_usize;
    let mut monkeys_one = Monkeys::new();
    let mut monkeys_two = Monkeys::new();
    let mut monkey_num = 0_usize;

    // For each Monkey
    let mut items = Vec::<usize>::new();
    let mut operation = Operation::Unknown;
    let mut operation_value = 0_usize;
    let mut test_value = 0_usize;
    let mut if_true = 0_usize;
    let mut if_false = 0_usize;
    let inspected_items = 0_usize;

    for mut line in input.lines() {
        line = line.trim();

        if line.is_empty() {
            monkeys_one.0.push(Monkey {
                num: monkey_num,
                items: items.clone(),
                operation,
                operation_value,
                test_value,
                if_true,
                if_false,
                inspected_items,
            });

            monkeys_two.0.push(Monkey {
                num: monkey_num,
                items: items.clone(),
                operation,
                operation_value,
                test_value,
                if_true,
                if_false,
                inspected_items,
            });

            items.clear();
            line_no = 1;
            monkey_num += 1;

            continue;
        }

        if line_no == 2 {
            for token in line.split_whitespace() {
                for item in token.split(',') {
                    if let Ok(val) = item.parse::<usize>() {
                        items.push(val);
                    }
                }
            }
        } else if line_no == 3 {
            for token in line.split_whitespace() {
                if token.eq("*") {
                    operation = Operation::Multiply;
                } else if token.eq("+") {
                    operation = Operation::Add;
                } else if token.eq("old") {
                    operation = Operation::Pow;
                } else if token.parse::<usize>().is_ok() {
                    operation_value = token.parse::<usize>().unwrap();
                }
            }
        } else if line_no == 4 {
            for token in line.split_whitespace() {
                if let Ok(val) = token.parse::<usize>() {
                    test_value = val;
                }
            }
        } else if line_no == 5 {
            for token in line.split_whitespace() {
                if let Ok(val) = token.parse::<usize>() {
                    if_true = val;
                }
            }
        } else if line_no == 6 {
            for token in line.split_whitespace() {
                if let Ok(val) = token.parse::<usize>() {
                    if_false = val;
                }
            }
        }

        line_no += 1;
    }

    // Push the last Monkey
    monkeys_one.0.push(Monkey {
        num: monkey_num,
        items: items.clone(),
        operation,
        operation_value,
        test_value,
        if_true,
        if_false,
        inspected_items,
    });

    monkeys_two.0.push(Monkey {
        num: monkey_num,
        items,
        operation,
        operation_value,
        test_value,
        if_true,
        if_false,
        inspected_items,
    });

    monkeys_one.do_part_one();
    monkeys_two.do_part_two();
}
