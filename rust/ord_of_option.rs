macro_rules! test {
    ($t:expr) => { println!("{}: {:?}", stringify!($t), $t); }
}
fn main() {
    test!(Some(1).cmp(&None));
    test!(None.cmp(&Some(1)));
}
