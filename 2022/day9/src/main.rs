use utils::*;

use day9::Map;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut part_one_map = Map::new("One".to_string(), 2);
    let mut part_two_map = Map::new("Two".to_string(), 10);

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        part_one_map.parse(line.to_string());
        part_two_map.parse(line.to_string());
    }

    part_one_map.do_part();
    part_two_map.do_part();
}
