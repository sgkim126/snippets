macro_rules! check {
    ($n:expr, $expected:expr) => {
        println!("{:?}", $n.to_be_bytes());
        assert_eq!($expected, $n.to_be_bytes());
    }
}

fn main() {
    check!(1u8, [1]);
    check!(1u16, [0, 1]);
    check!(1u32, [0, 0, 0, 1]);
    check!(1u64, [0, 0, 0, 0, 0, 0, 0, 1]);
    check!(1u128, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}
