use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn solve_part_i() -> u32 {
    let elf_cals_list: Vec<u32> = get_elf_calories_list();
    
    // attempt to get value of the elf carrying the most calories
    let res: Option<&u32> = elf_cals_list.iter().max();

    return match res {
        Some(val) => *val,
        None => {
            panic!("there are no elves on this fruit expedition");
        }
    };
}

pub fn solve_part_ii() -> u32 {
    let elf_cals_list: Vec<u32> = get_elf_calories_list();
    
    let mut top_three_arr: [u32; 3] = [0, 0, 0];
    let mut ans = 0;

    // calculate combined total calories of top 3 elves
    for cals in elf_cals_list {
        let mut min_i: usize = 0;
        let mut has_new_top_three: bool = false;

        // check if current elf total calories is greater any of the existing top 3 elf total calories
        for i in 0..top_three_arr.len() {
            if cals > top_three_arr[i] {
                if !has_new_top_three || top_three_arr[i] < top_three_arr[min_i] {
                    min_i = i;
                    has_new_top_three = true;
                }
            }
        }

        // check if there is a new top 3 elf
        if has_new_top_three {
            // update combined total calories of top 3 elves
            ans += cals - top_three_arr[min_i];

            // replace 1 of the top 3 elves total calories
            top_three_arr[min_i] = cals;
        }
    }

    return ans;
}

fn get_elf_calories_list() -> Vec<u32> {
    let file_path: String = "./resources/twenty_twenty_two/day_one/input.txt".to_string();

    let file: File = File::open(file_path).expect("file does not exist");
    let buf_reader: BufReader<File> = BufReader::new(file);

    let mut elf_cals_list: Vec<u32> = Vec::new();
    let mut total_elf_cals: u32 = 0;
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

                        total_elf_cals += val
                    }
                    Err(_) => {
                        // if new elf calories add previous elf calories to list and reset elf calories counter
                        elf_cals_list.push(total_elf_cals);
                        total_elf_cals = 0;
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
        elf_cals_list.push(total_elf_cals);
    }

    return elf_cals_list;
}