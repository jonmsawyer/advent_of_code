use utils::*;

use day8::Forest;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut forest = Forest::new();

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        forest.parse(line.to_string());
    }

    println!("{}", forest);

    forest.do_part_1();
    forest.do_part_2();
}
