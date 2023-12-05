use r_5;
use std::fs;

#[test]
fn it_compute_1() {
    let file_path = "r_5/data/sample_1.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("Failed to read {}", file_path));
    assert_eq!(35, r_5::part_1(&contents));
}
#[test]
fn it_compute_2() {
    let file_path = "r_5/data/sample_2.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("Failed to read {}", file_path));
    assert_eq!(46, r_5::part_2(&contents));
}
