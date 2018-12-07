fn main() {
    let a = String::new();
    let b = "".to_string();
    assert_eq!(a, b);
    assert_eq!(a.as_str(), b.as_str());
}
