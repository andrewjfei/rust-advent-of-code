use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn defeats(&self) -> Move {
        return match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn loses_to(&self) -> Move {
        return match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn compare(&self, opp_mv: &Move) -> Outcome {
        if &self.defeats() == opp_mv {
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
    let round_strat_list: Vec<String> = get_sub_str_list();
    return calc_total_score(round_strat_list, true);
}

pub fn solve_part_ii() -> u32 {
    let round_strat_list: Vec<String> = get_sub_str_list();
    return calc_total_score(round_strat_list, false);
}

fn get_sub_str_list() -> Vec<String> {
    let file_path: &str = "./resources/twenty_twenty_two/day_two/input.txt";

    let str: String = read_to_string(file_path).expect("file does not exist");

    // split string into per round strategies
    let sub_str_list: Vec<String> = str
        .split("\n")
        .map(|sub_str| sub_str.to_string())
        .collect();

    return sub_str_list;
}

fn calc_total_score(round_strat_list: Vec<String>, is_part_i: bool) -> u32 {
    let mut total_score = 0;

    for strat_str in round_strat_list {
        // split round strategy into opponent and user moves
        let char_list: Vec<char> = strat_str
            .split(" ")
            .map(|char_str| char_str.chars().next().expect("failed to convert string to character"))
            .collect();

        let opp_mv: char = *char_list.get(0).expect("predicted opponent move does not exist");
        let usr_mv: char = *char_list.get(1).expect("predicted user move does not exist");

        // calculate round score
        total_score += match is_part_i {
            true => calc_round_score_part_i(opp_mv, usr_mv),
            false => calc_round_score_part_ii(opp_mv, usr_mv),
        };
    }

    return total_score;
}

fn calc_round_score_part_i(opp_mv: char, usr_mv: char) -> u32 {
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

fn calc_round_score_part_ii(opp_mv: char, outcome: char) -> u32 {
    let mut round_score: u32 = 0;

    // convert move character into move enum
    let opp_mv: Move = convert_to_mv(opp_mv);
    let outcome: Outcome = convert_to_outcome(outcome);

    // append outcome score
    round_score += outcome.get_score();

    // append move score
    round_score += get_usr_mv(opp_mv, outcome).get_score();

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

fn convert_to_outcome(outcome: char) -> Outcome {
    return match outcome {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("invalid outcome character"),
    };
}

fn get_usr_mv(opp_mv: Move, outcome: Outcome) -> Move {
    return match outcome {
        Outcome::Win => opp_mv.loses_to(),
        Outcome::Draw => opp_mv,
        Outcome::Lose => opp_mv.defeats(),
    }
}
