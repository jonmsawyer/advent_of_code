use std::cmp::{Ordering, PartialOrd};

use utils::*;

#[derive(PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (RPS::Rock, RPS::Rock) => Ordering::Equal,
            (RPS::Rock, RPS::Paper) => Ordering::Less,
            (RPS::Rock, RPS::Scissors) => Ordering::Greater,
            (RPS::Paper, RPS::Rock) => Ordering::Greater,
            (RPS::Paper, RPS::Paper) => Ordering::Equal,
            (RPS::Paper, RPS::Scissors) => Ordering::Less,
            (RPS::Scissors, RPS::Rock) => Ordering::Less,
            (RPS::Scissors, RPS::Paper) => Ordering::Greater,
            (RPS::Scissors, RPS::Scissors) => Ordering::Equal,
        }
    }
}

fn score(player1: &RPS, player2: &RPS) -> usize {
    let mut score = match player2 {
        RPS::Rock => 1_usize,
        RPS::Paper => 2_usize,
        RPS::Scissors => 3_usize,
    };

    match player2.cmp(player1) {
        Ordering::Equal => score += 3_usize,
        Ordering::Greater => score += 6_usize,
        Ordering::Less => {}
    }

    score
}

fn match_players(sign1: &str, sign2: &str) -> Option<(RPS, RPS)> {
    match (sign1, sign2) {
        ("A", "X") => Some((RPS::Rock, RPS::Rock)),
        ("A", "Y") => Some((RPS::Rock, RPS::Paper)),
        ("A", "Z") => Some((RPS::Rock, RPS::Scissors)),
        ("B", "X") => Some((RPS::Paper, RPS::Rock)),
        ("B", "Y") => Some((RPS::Paper, RPS::Paper)),
        ("B", "Z") => Some((RPS::Paper, RPS::Scissors)),
        ("C", "X") => Some((RPS::Scissors, RPS::Rock)),
        ("C", "Y") => Some((RPS::Scissors, RPS::Paper)),
        ("C", "Z") => Some((RPS::Scissors, RPS::Scissors)),
        _ => None,
    }
}

fn match_signs(sign1: &str, sign2: &str) -> Option<(RPS, RPS)> {
    match (sign1, sign2) {
        ("A", "X") => Some((RPS::Rock, RPS::Scissors)), // Rock beats Scissors (X = lose)
        ("A", "Y") => Some((RPS::Rock, RPS::Rock)),     // Rock draws Rock (Y = draw)
        ("A", "Z") => Some((RPS::Rock, RPS::Paper)),    // Rock loses to Paper (Z = win)
        ("B", "X") => Some((RPS::Paper, RPS::Rock)),
        ("B", "Y") => Some((RPS::Paper, RPS::Paper)),
        ("B", "Z") => Some((RPS::Paper, RPS::Scissors)),
        ("C", "X") => Some((RPS::Scissors, RPS::Paper)),
        ("C", "Y") => Some((RPS::Scissors, RPS::Scissors)),
        ("C", "Z") => Some((RPS::Scissors, RPS::Rock)),
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
        if let Some((player1, player2)) = match_players(line_vec[0], line_vec[1]) {
            player_score_part_one += score(&player1, &player2);
        }
        if let Some((player1, player2)) = match_signs(line_vec[0], line_vec[1]) {
            player_score_part_two += score(&player1, &player2);
        }
    }

    println!("Part One: Player score is: {}", player_score_part_one);
    println!("Part Two: Player score is: {}", player_score_part_two);
}
