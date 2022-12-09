use std::collections::HashSet;

use utils::*;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");

    for mut line in input.lines() {
        line = line.trim_end();
        if line.is_empty() {
            continue;
        }

        let line: Vec<char> = line.chars().collect();
        let mut set = HashSet::<char>::new();

        //
        // Part One
        //

        for (idx, line) in line.windows(4).enumerate() {
            for c in line {
                set.insert(*c);
            }

            if set.len() == 4 {
                println!("Start-of-packet marker is: {}", idx + 4);
                break;
            }

            set.clear();
        }

        //
        // Part Two
        //

        for (idx, line) in line.windows(14).enumerate() {
            for c in line {
                set.insert(*c);
            }

            if set.len() == 14 {
                println!("Start-of-message marker is: {}", idx + 14);
                break;
            }

            set.clear();
        }
    }
}
