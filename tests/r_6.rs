use r_6;

#[test]
fn it_compute_1() {
    assert_eq!(288 as usize, r_6::compute_1(String::from("r_6/data/sample_1.txt")));
}
#[test]
fn it_compute_2() {
    assert_eq!(71503 as usize, r_6::compute_2(String::from("r_6/data/sample_1.txt")));
}
