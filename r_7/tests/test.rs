use r_7;

#[test]
fn it_compute_1() {
    assert_eq!(6440, r_7::compute_1(String::from("data/sample_1.txt")));
}
#[test]
fn it_compute_2() {
    assert_eq!(5905, r_7::compute_2(String::from("data/sample_1.txt")));
}
