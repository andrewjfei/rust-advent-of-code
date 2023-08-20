mod twenty_twenty_two;

fn main() {
    println!();
    println!("==================== 2022 ====================");
    println!();
    println!(
        "Day 1 Part I\tAnswer: {}",
        twenty_twenty_two::day_one::solve_part_i()
    );
    println!(
        "Day 1 Part II\tAnswer: {}",
        twenty_twenty_two::day_one::solve_part_ii()
    );
    println!();
    println!(
        "Day 2 Part I\tAnswer: {}",
        twenty_twenty_two::day_two::solve_part_i()
    );
    println!(
        "Day 2 Part II\tAnswer: {}",
        twenty_twenty_two::day_two::solve_part_ii()
    );
    println!();
}
