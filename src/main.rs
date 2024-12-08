mod solutions;
use solutions::{day_1::day_1, day_2::day_2};


fn main() {
    // Day 1
    let day_1_path = "src/inputs/day_1_part_1.txt";
    let day_1_results = day_1(day_1_path);
    println!("Day 1, Part 1: {}", day_1_results.part_1);
    println!("Day 1, Part 2: {}", day_1_results.part_2);

    // Day 2
    let day_2_path = "src/inputs/day_2_part_1.txt";
    let day_2_results = day_2(day_2_path);
    println!("Day 2, Part 1: {}", day_2_results.part_1);
    println!("Day 2, Part 2: {}", day_2_results.part_2);
}
