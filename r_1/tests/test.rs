use r_1;

#[test]
fn it_compute_1() {
    assert_eq!(142, r_1::compute_1(String::from("data/sample_1.txt")));
}
#[test]
fn it_compute_2() {
    assert_eq!(281, r_1::compute_2(String::from("data/sample_2.txt")));
}
