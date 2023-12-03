use r_3;

#[test]
fn it_compute_1() {
    assert_eq!(4361, r_3::compute_1(String::from("r_3/data/sample_1.txt")));
}
#[test]
fn it_compute_2() {
    assert_eq!(467835, r_3::compute_2(String::from("r_3/data/sample_2.txt")));
}
