pub mod packet;

use utils::*;

use day13::Packet;

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut packet = Packet::new();

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        if packet.parse(line).is_err() {
            println!("Error reading packet: {}", line);
        }
    }

    if packet.compute_indicies().is_err() {
        println!("Could not compute indicies.");
    }

    if packet.sort_packets().is_err() {
        println!("Could not sort packets.");
    }

    if packet.compute_sorted_indicies().is_err() {
        println!("Could not compute sorted indicies.");
    }

    packet.do_part_one();
    packet.do_part_two();
}
