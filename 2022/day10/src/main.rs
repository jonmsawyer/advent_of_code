use utils::*;

use day10::Processor;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");

    let mut processor = Processor::new();

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        processor.parse(line.to_string());
    }

    processor.do_part();
}
