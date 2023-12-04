extern crate r_1;
extern crate r_2;
extern crate r_3;
extern crate r_4;

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
    println!(
        "day 3 : {} / {}",
        r_3::compute_1(String::from("r_3/data/input_1.txt")),
        r_3::compute_2(String::from("r_3/data/input_2.txt"))
    );
    println!(
        "day 4 : {} / {}",
        r_4::compute_1(String::from("r_4/data/input_1.txt")),
        r_4::compute_2(String::from("r_4/data/input_2.txt"))
    );
}
