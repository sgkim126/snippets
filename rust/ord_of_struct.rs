macro_rules! test {
    ($t:expr) => { println!("{}: {:?}", stringify!($t), $t); }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct S {
    a: usize,
    b: usize,
}

fn main() {
    test!(S { a: 1, b: 2 }.cmp(&S { a: 1, b: 1 }));
    test!(S { a: 1, b: 2 }.cmp(&S { a: 1, b: 2 }));
    test!(S { a: 1, b: 2 }.cmp(&S { a: 1, b: 3 }));
    test!(S { a: 1, b: 2 }.cmp(&S { a: 0, b: 2 }));
    test!(S { a: 1, b: 2 }.cmp(&S { a: 1, b: 2 }));
    test!(S { a: 1, b: 2 }.cmp(&S { a: 2, b: 2 }));
    test!(S { a: 1, b: 2 }.cmp(&S { a: 0, b: 3 }));
    test!(S { a: 1, b: 2 }.cmp(&S { a: 2, b: 1 }));
}
