use r_2;

#[test]
fn it_compute_1() {
    assert_eq!(8, r_2::compute_1(String::from("r_2/data/sample_1.txt")));
}
#[test]
fn it_compute_2() {
    assert_eq!(2286, r_2::compute_2(String::from("r_2/data/sample_2.txt")));
}
