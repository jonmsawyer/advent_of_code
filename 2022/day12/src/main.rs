use utils::*;

use day12::Map;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut map = Map::new();

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        map.parse(line.to_string());
    }

    map.calculate_start_and_end_pos();

    map.do_part_one();
    map.do_part_two();
}
