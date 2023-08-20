use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn defeats(&self, opp_mv: &Move) -> bool {
        return (self == &Move::Rock && opp_mv == &Move::Scissors) 
            || (self == &Move::Paper && opp_mv == &Move::Rock) 
            || (self == &Move::Scissors && opp_mv == &Move::Paper);
    }

    fn compare(&self, opp_mv: &Move) -> Outcome {
        if self.defeats(&opp_mv) {
            return Outcome::Win;
        } else if self == opp_mv {
            return Outcome::Draw;
        } else {
            return Outcome::Lose;
        }
    }

    fn get_score(&self) -> u32 {
        return match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn get_score(&self) -> u32 {
        return match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

pub fn solve_part_i() -> u32 {
    let file_path: &str = "./resources/twenty_twenty_two/day_two/input.txt";

    let str: String = read_to_string(file_path).expect("file does not exist");

    // split string into round stratgies
    let sub_str_list: Vec<String> = str
        .split("\n")
        .map(|sub_str| sub_str.to_string())
        .collect();

    let mut total_score = 0;

    for sub_str in sub_str_list {
        // split round strategy into opponent and user moves
        let char_list: Vec<char> = sub_str
            .split(" ")
            .map(|char_str| char_str.chars().next().expect("failed to convert string to character"))
            .collect();

        let opp_mv: char = *char_list.get(0).expect("predicted opponent move does not exist");
        let usr_mv: char = *char_list.get(1).expect("predicted user move does not exist");

        // calculate round score
        total_score += calc_round_score(opp_mv, usr_mv);
    }

    return total_score;
}

fn calc_round_score(opp_mv: char, usr_mv: char) -> u32 {
    let mut round_score: u32 = 0;

    // convert move character into move enum
    let opp_mv: Move = convert_to_mv(opp_mv);
    let usr_mv: Move = convert_to_mv(usr_mv);

    // append outcome score
    round_score += usr_mv.compare(&opp_mv).get_score();

    // append move score
    round_score += usr_mv.get_score();

    return round_score;
}

fn convert_to_mv(mv: char) -> Move {
    if mv == 'A' || mv == 'X' {
        return Move::Rock;
    } else if mv == 'B' || mv == 'Y' {
        return Move::Paper;
    } else if mv == 'C' || mv == 'Z' {
        return Move::Scissors;
    } else {
        panic!("invalid move character")
    }
}
