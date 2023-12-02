extern crate r_1;
extern crate r_2;

fn main() {
    println!(
        "day 1 : {} / {}",
        r_1::compute_1(String::from("r_1/data/input_1.txt")),
        r_1::compute_2(String::from("r_1/data/input_2.txt"))
    );
    println!(
        "day 2 : {} / {}",
        r_2::compute_1(String::from("r_2/data/input_1.txt")),
        r_2::compute_2(String::from("r_2/data/input_2.txt"))
    );
}
