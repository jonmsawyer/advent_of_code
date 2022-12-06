use utils::*;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");

    let mut elf = 0_usize;
    let mut calories = vec![elf];

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            elf += 1_usize;
            calories.push(0_usize);
            continue;
        }

        calories[elf] += line.parse::<usize>().expect("Expected integer input.");
    }

    calories.sort();

    //
    // Part One
    //

    let mut result = calories.pop().expect("Expected value from end of vector.");

    println!(
        "The most amount of Calories an Elf is carrying is: {}",
        result
    );

    //
    // Part Two
    //
    
    result += calories.iter().rev().take(2).sum::<usize>();

    println!(
        "The total calories of the top three Calorie-carrying Elves are: {}",
        result
    );
}
