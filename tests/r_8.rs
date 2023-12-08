use r_8;

#[test]
fn it_compute_1() {
    assert_eq!(2, r_8::compute_1(String::from("r_8/data/sample_1.txt")));
}
#[test]
fn it_compute_2() {
    assert_eq!(6, r_8::compute_2(String::from("r_8/data/sample_2.txt")));
}
