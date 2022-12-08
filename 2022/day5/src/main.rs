use utils::*;

const STACKS_LEN: usize = 9;
const COL_MAX: usize = 35;

fn get_crates(line: &str) -> Vec<Option<char>> {
    let mut col_count = 0_usize;
    let mut crates = Vec::<Option<char>>::new();
    for c in line.chars() {
        if col_count % 4 == 1 {
            if ('1'..='9').contains(&c) {
                crates.clear();
                return crates;
            }
            else if ('A'..='Z').contains(&c) {
                crates.push(Some(c));
            }
            else if c == ' ' {
                crates.push(None);
            }
        }

        col_count += 1;
    }

    while col_count < COL_MAX {
        crates.push(None);
        col_count += 4;
    }

    crates
}

/// Debug.
fn print_stacks(stacks: &[Vec<Option<char>>]) {
    let mut lens = Vec::<usize>::new();
    let mut max = 0_usize;

    for stack in stacks {
        lens.push(stack.len());
        if stack.len() > max {
            max = stack.len();
        }
    }

    for idx in 0..max {
        let cur_idx = max - 1 - idx;
        for (stack_idx, stack) in stacks.iter().enumerate() {
            if (lens[stack_idx] as isize) - (max as isize) + (idx as isize) < 0 {
                print!("    ");
                continue;
            }

            if let Some(p_char) = stack[cur_idx] {
                print!("[{}] ", p_char);
            }
        }
        println!();
    }

    for (idx, _) in stacks.iter().enumerate() {
        print!(" {}  ", idx + 1);
    }

    println!();
}

fn parse_move(line: &str) -> Vec<usize> {
    let the_move: Vec<usize> = line
        .split(' ')
        .filter_map(|token| match token.parse::<usize>() {
            Ok(num) => Some(num),
            _ => None,
        })
        .collect();

    the_move
}

fn perform_operation_1(stacks: &mut [Vec<Option<char>>], operation: &[usize]) {
    for _ in 0..operation[0] {
        let item = stacks[operation[1] - 1].pop();
        if let Some(item) = item {
            stacks[operation[2] - 1].push(item);
        }
    }
}

fn perform_operation_2(stacks: &mut [Vec<Option<char>>], operation: &[usize]) {
    let mut stack = Vec::<Option<char>>::new();

    for _ in 0..operation[0] {
        let item = stacks[operation[1] - 1].pop();
        if let Some(item) = item {
            stack.push(item);
        }
    }

    for _ in 0..stack.len() {
        let item = stack.pop();
        if let Some(item) = item {
            stacks[operation[2] - 1].push(item);
        }
    }
}

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut stacks1 = Vec::<Vec<Option<char>>>::with_capacity(STACKS_LEN); // because of our static input
    let mut stacks2 = Vec::<Vec<Option<char>>>::with_capacity(STACKS_LEN); // because of our static input
    // Parsing state. `0` if parsing the initial stacks of crates; `1` if parsing move list.
    let mut state = 0_usize;

    // Initialize our stacks.
    for _ in 0..STACKS_LEN {
        stacks1.push(Vec::<Option<char>>::new());
        stacks2.push(Vec::<Option<char>>::new());
    }

    for mut line in input.lines() {
        line = line.trim_end();
        if line.is_empty() {
            state = 1;
            continue;
        }

        // Parse the stacks.
        if state == 0 {
            let crates = get_crates(line);

            if crates.is_empty() {
                continue;
            }

            for (idx, crate_) in crates.iter().enumerate() {
                if stacks1[idx].is_empty() {
                    if crate_.is_some() {
                        stacks1[idx].push(*crate_);
                    }
                } else {
                    if let Some(crate_) = crate_ {
                        if ('1'..='9').contains(crate_) {
                            continue;
                        }
                    }

                    if crate_.is_some() {
                        stacks1[idx].insert(0, *crate_);
                    }
                }

                if stacks2[idx].is_empty() {
                    if crate_.is_some() {
                        stacks2[idx].push(*crate_);
                    }
                } else {
                    if let Some(crate_) = crate_ {
                        if ('1'..='9').contains(crate_) {
                            continue;
                        }
                    }

                    if crate_.is_some() {
                        stacks2[idx].insert(0, *crate_);
                    }
                }
            }
        }

        // Parse the move list.
        else if state == 1 {
            let op = parse_move(line);

            //
            // Part One
            //

            perform_operation_1(&mut stacks1, &op);

            //
            // Part Two
            //

            perform_operation_2(&mut stacks2, &op);
        }
    }

    println!("Stacks 1:");
    print_stacks(&stacks1);

    println!();

    println!("Stacks 2:");
    print_stacks(&stacks2);
}
