use std::collections::HashSet;

use utils::*;

fn str_to_set(string: &str) -> HashSet<char> {
    let mut set = HashSet::<char>::new();

    for c in string.chars() {
        set.insert(c);
    }

    set
}

fn priority(c: &char) -> u32 {
    if ('a'..='z').contains(c) {
        *c as u32 - 96 // 1..=26
    }
    else if ('A'..='Z').contains(c) {
        *c as u32 - 38 // 27..=52
    }
    else {
        0
    }
}

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut compartment_total = 0_u32;
    let mut group_total = 0_u32;
    let mut sacks = Vec::<HashSet<char>>::new();

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        //
        // Part One
        //
        let (half1, half2) = line.split_at(line.len() / 2);

        let set1 = str_to_set(half1);
        let set2 = str_to_set(half2);

        for c in set1.intersection(&set2) {
            compartment_total += priority(c);
        }

        //
        // Part Two
        //
        sacks.push(str_to_set(line));
        if sacks.len() == 3 { // Evaluate every 3 rucksacks.
            let mut badge = HashSet::<char>::new();
            // Rust doesn't yet support the intersection of multiple sets, so I do
            // it manually here.
            for c in sacks[0].intersection(&sacks[1]) {
                badge.insert(*c);
            }
            for c in badge.intersection(&sacks[2]) {
                group_total += priority(c)
            }
            sacks.clear();
        }
    }

    println!("The sum of priorities is: {}", compartment_total);
    println!("The sum of the group priorities is: {}", group_total);
}
