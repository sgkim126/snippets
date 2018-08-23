fn main() {
    println!("{:?}", "a b c\nd\te  f".split_whitespace().map(String::from).collect::<Vec<_>>());
    println!("{:?}", "a b c\nd\te  f".to_string().split_whitespace().map(String::from).collect::<Vec<_>>());
}
