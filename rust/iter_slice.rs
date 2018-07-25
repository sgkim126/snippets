fn contains_zero(values: &[i32]) -> bool {
    values.iter().any(|v| {
        v == &0
    })
}

fn main() {
    assert!(!contains_zero(&[1, 2, 3, 4, 5]));
    assert!(contains_zero(&[0, 2, 3, 4, 5]));
    assert!(contains_zero(&[1, 2, 0, 4, 5]));
    assert!(contains_zero(&[1, 2, 3, 4, 0]));
}
