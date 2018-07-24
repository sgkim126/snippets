use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

fn concat(a: String, b: String) -> String {
    format!("{}{}", a, b)
}

fn main() {
    let a: &'static str = "ab";
    let b: String = concat("a".to_string(), "b".to_string());
    let c: &str = b.as_str();

    let mut a_hasher = DefaultHasher::new();
    a.hash(&mut a_hasher);

    let mut b_hasher = DefaultHasher::new();
    b.hash(&mut b_hasher);

    let mut c_hasher = DefaultHasher::new();
    c.hash(&mut c_hasher);

    assert_eq!(a_hasher.finish(), b_hasher.finish());
    assert_eq!(a_hasher.finish(), c_hasher.finish());
}
