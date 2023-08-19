use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn solve() -> u32 {
    let file_path: String = "./resources/twenty_twenty_two/day_one/input.txt".to_string();

    let file: File = File::open(file_path).expect("file does not exist");
    let buf_reader: BufReader<File> = BufReader::new(file);

    let mut elf_calories_list: Vec<u32> = Vec::new();
    let mut total_elf_calories: u32 = 0;
    let mut is_counting: bool = false;

    // read file line by line
    for line in buf_reader.lines() {
        match line {
            Ok(str) => {
                // attempt to parse string as an unsigned integer
                let res: Result<u32, ParseIntError> = str.parse::<u32>();

                match res {
                    Ok(val) => {
                        if !is_counting {
                            is_counting = true;
                        }

                        total_elf_calories += val
                    }
                    Err(_) => {
                        // if new elf calories add previous elf calories to list and reset elf calories counter
                        elf_calories_list.push(total_elf_calories);
                        total_elf_calories = 0;
                        is_counting = false;
                    }
                }
            }
            Err(_) => {
                // do nothing
            }
        }
    }

    if is_counting {
        // add final elf calories to list
        elf_calories_list.push(total_elf_calories);
    }

    // attempt to get value of the elf carrying the most calories
    let res: Option<&u32> = elf_calories_list.iter().max();

    return match res {
        Some(val) => *val,
        None => {
            panic!("there are no elves on this fruit expedition");
        }
    };
}
