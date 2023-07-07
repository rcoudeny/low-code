fn main() {}

#[test]
fn only_backend_test() {
    assert_eq!(1, 1, "1 is not equal to 1");
}

#[test]
fn not_the_only_backend_test() {
    assert_eq!(2, 1, "2 is not equal to 1");
}
