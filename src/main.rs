fn main() {
    reverse_string;
}
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}
#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("rust"), "tsur");
    assert_eq!(reverse_string(""), "");
    assert_eq!(reverse_string("a"), "a");
    assert_eq!(reverse_string("12345"), "54321");
    assert_eq!(reverse_string("uUu"), "uUu");
}
