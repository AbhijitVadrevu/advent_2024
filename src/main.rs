mod solutions;
use solutions::day_1::day_1;


fn main() {
    // Day 1
    let file_path = "src/inputs/day_1_part_1.txt";
    let day_1_results = day_1(&file_path);
    println!("Day 1, Part 1: {}", day_1_results.part_1);
    println!("Day 1, Part 2: {}", day_1_results.part_2);
}
