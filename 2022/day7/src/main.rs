use utils::*;

use day7::FileSystem;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut filesystem = FileSystem::new();

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        filesystem.parse(line.to_string());
    }

    println!("{}", filesystem);

    filesystem.do_part_1();
    filesystem.do_part_2();
}
