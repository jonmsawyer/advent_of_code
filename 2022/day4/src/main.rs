use utils::*;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut contained_count = 0_usize;
    let mut overlapping_count = 0_usize;

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let pair: Vec<&str> = line.split(',').collect();
        let range1: Vec<usize> = pair[0]
            .split('-')
            .map(|el| el.parse::<usize>().expect("Must be positive integer."))
            .collect();
        let range2: Vec<usize> = pair[1]
            .split('-')
            .map(|el| el.parse::<usize>().expect("Must be positive integer."))
            .collect();

        //
        // Part One
        //
        // Find contained sections.
        if (range1[0] <= range2[0] && range1[1] >= range2[1])
            || (range2[0] <= range1[0] && range2[1] >= range1[1])
        {
            contained_count += 1
        }

        //
        // Part Two
        //
        // Find overlapping sections.
        if (range1[0] >= range2[0] && range1[0] <= range2[1])
            || (range1[1] >= range2[0] && range1[1] <= range2[1])
            || (range2[0] >= range1[0] && range2[0] <= range1[1])
            || (range2[1] >= range1[0] && range2[1] <= range1[1])
        {
            overlapping_count += 1
        }
    }

    println!("The number of contained sections is: {}", contained_count);
    println!(
        "The number of overlapping sections is: {}",
        overlapping_count
    );
}
