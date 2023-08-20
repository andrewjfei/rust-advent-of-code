mod year_2022;

fn main() {
    println!();
    println!("==================== 2022 ====================");
    println!();
    println!("Day 1 Part I\tAnswer: {}", year_2022::day_1::solve_part_i());
    println!(
        "Day 1 Part II\tAnswer: {}",
        year_2022::day_1::solve_part_ii()
    );
    println!();
    println!("Day 2 Part I\tAnswer: {}", year_2022::day_2::solve_part_i());
    println!(
        "Day 2 Part II\tAnswer: {}",
        year_2022::day_2::solve_part_ii()
    );
    println!();
    println!("Day 3 Part I\tAnswer: {}", year_2022::day_3::solve_part_i());
    println!();
}
