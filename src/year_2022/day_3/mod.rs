use std::fs::read_to_string;

use regex::Regex;

pub fn solve_part_i() -> u32 {
    let file_path: String = String::from("./resources/year_2022/day_3/input.txt");

    let str: String = read_to_string(file_path).expect("file does not exist");

    let mut combined_item_priorities: u32 = 0;

    for line in str.lines() {
        let (left, right): (&str, &str) = seperate_compartments(line);

        let dup_item: char = find_dup_item(left, right);

        combined_item_priorities += get_item_priority(dup_item);
    }

    return combined_item_priorities;
}

fn seperate_compartments(str: &str) -> (&str, &str) {
    // check if number of items in each rucksack compartment is equal
    if str.len() % 2 != 0 {
        panic!("rucksack compartments are not even");
    }

    // split ruck sack by compartment
    return str.split_at(str.len() / 2);
}

fn find_dup_item(left: &str, right: &str) -> char {
    // find duplicate item in rucksack compartments
    for char in left.chars() {
        if right.contains(char) {
            return char;
        }
    }

    // panic if rucksack compartment is empty
    panic!("rucksack compartment is empty");
}

fn get_item_priority(item: char) -> u32 {
    let lowercase_pattern: Regex =
        Regex::new(r"^[a-z]$").expect("failed to create a-z regex pattern");
    let uppercase_pattern: Regex =
        Regex::new(r"^[A-Z]$").expect("failed to create A-Z regex pattern");

    let str_item: &str = &item.to_string();

    if lowercase_pattern.is_match(str_item) {
        // calculate priorities 1-26 for items a-z
        return item as u32 - 96;
    } else if uppercase_pattern.is_match(str_item) {
        // calculate priorities 27-52 for items A-Z
        return item as u32 - 38;
    } else {
        panic!("invalid item character")
    }
}
