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

    let mut result = calories.pop().expect("Expected value from end of vector.");

    println!(
        "Part One: The most amount of Calories an Elf is carrying is: {}",
        result
    );

    result += calories.iter().rev().take(2).sum::<usize>();

    println!(
        "Part Two: The total calories of the top three Calorie-carrying Elves are: {}",
        result
    );
}
