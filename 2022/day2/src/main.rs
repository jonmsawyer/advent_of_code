use std::cmp::{Ordering, PartialOrd};

use utils::*;

#[derive(PartialEq, Eq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RockPaperScissors {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => Ordering::Equal,
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => Ordering::Less,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => Ordering::Greater,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => Ordering::Greater,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => Ordering::Equal,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => Ordering::Less,
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => Ordering::Less,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => Ordering::Greater,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => Ordering::Equal,
        }
    }
}

fn score(player1: &RockPaperScissors, player2: &RockPaperScissors) -> usize {
    let mut score = match player2 {
        RockPaperScissors::Rock => 1_usize,
        RockPaperScissors::Paper => 2_usize,
        RockPaperScissors::Scissors => 3_usize,
    };

    match player2.cmp(player1) {
        Ordering::Equal => score += 3_usize,
        Ordering::Greater => score += 6_usize,
        Ordering::Less => {}
    }

    score
}

fn match_players(sign1: &str, sign2: &str) -> Option<(RockPaperScissors, RockPaperScissors)> {
    match (sign1, sign2) {
        ("A", "X") => Some((RockPaperScissors::Rock, RockPaperScissors::Rock)),
        ("A", "Y") => Some((RockPaperScissors::Rock, RockPaperScissors::Paper)),
        ("A", "Z") => Some((RockPaperScissors::Rock, RockPaperScissors::Scissors)),
        ("B", "X") => Some((RockPaperScissors::Paper, RockPaperScissors::Rock)),
        ("B", "Y") => Some((RockPaperScissors::Paper, RockPaperScissors::Paper)),
        ("B", "Z") => Some((RockPaperScissors::Paper, RockPaperScissors::Scissors)),
        ("C", "X") => Some((RockPaperScissors::Scissors, RockPaperScissors::Rock)),
        ("C", "Y") => Some((RockPaperScissors::Scissors, RockPaperScissors::Paper)),
        ("C", "Z") => Some((RockPaperScissors::Scissors, RockPaperScissors::Scissors)),
        _ => None,
    }
}

fn match_signs(sign1: &str, sign2: &str) -> Option<(RockPaperScissors, RockPaperScissors)> {
    match (sign1, sign2) {
        ("A", "X") => Some((RockPaperScissors::Rock, RockPaperScissors::Scissors)), // Rock beats Scissors (X = lose)
        ("A", "Y") => Some((RockPaperScissors::Rock, RockPaperScissors::Rock)), // Rock draws Rock (Y = draw)
        ("A", "Z") => Some((RockPaperScissors::Rock, RockPaperScissors::Paper)), // Rock loses to Paper (Z = win)
        ("B", "X") => Some((RockPaperScissors::Paper, RockPaperScissors::Rock)),
        ("B", "Y") => Some((RockPaperScissors::Paper, RockPaperScissors::Paper)),
        ("B", "Z") => Some((RockPaperScissors::Paper, RockPaperScissors::Scissors)),
        ("C", "X") => Some((RockPaperScissors::Scissors, RockPaperScissors::Paper)),
        ("C", "Y") => Some((RockPaperScissors::Scissors, RockPaperScissors::Scissors)),
        ("C", "Z") => Some((RockPaperScissors::Scissors, RockPaperScissors::Rock)),
        _ => None,
    }
}

fn main() {
    let input = read_puzzle_input!().expect("Could not get input.");
    let mut player_score_part_one = 0_usize;
    let mut player_score_part_two = 0_usize;

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let line_vec: Vec<&str> = line.split_whitespace().collect();

        //
        // Part One
        //

        if let Some((player1, player2)) = match_players(line_vec[0], line_vec[1]) {
            player_score_part_one += score(&player1, &player2);
        }

        //
        // Part Two
        //

        if let Some((player1, player2)) = match_signs(line_vec[0], line_vec[1]) {
            player_score_part_two += score(&player1, &player2);
        }
    }

    println!("Part One: Player score is: {}", player_score_part_one);
    println!("Part Two: Player score is: {}", player_score_part_two);
}
