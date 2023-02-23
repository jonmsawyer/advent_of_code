use utils::*;

use day10::Processor;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    
    let mut processor = Processor {
        part: "One".to_string(),
        x: 1,
        ops: 0,
        signal_strength: Vec::<isize>::new(),
        crt_buffer: Vec::<char>::new(),
        pixel: 0,
    };

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        processor.parse(line.to_string());
    }

    processor.do_part();
}
