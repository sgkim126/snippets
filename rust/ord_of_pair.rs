macro_rules! test {
    ($t:expr) => { println!("{}: {:?}", stringify!($t), $t); }
}

fn main() {
    test!((1, 2).cmp(&(1, 1)));
    test!((1, 2).cmp(&(1, 2)));
    test!((1, 2).cmp(&(1, 3)));
    test!((1, 2).cmp(&(0, 2)));
    test!((1, 2).cmp(&(1, 2)));
    test!((1, 2).cmp(&(2, 2)));
    test!((1, 2).cmp(&(0, 3)));
    test!((1, 2).cmp(&(2, 1)));
}
