use r_4;

#[test]
fn it_compute_1() {
    assert_eq!(13, r_4::compute_1(String::from("data/sample_1.txt")));
}
#[test]
fn it_compute_2() {
    assert_eq!(30, r_4::compute_2(String::from("data/sample_2.txt")));
}
