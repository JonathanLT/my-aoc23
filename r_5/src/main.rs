use r_5;
use std::fs;

fn main() {
    let file_path = "data/input_1.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("Failed to read {}", file_path));

    println!("{} / {}", r_5::part_1(&contents), r_5::part_2(&contents));
}
