macro_rules! check {
    ($n:expr, $expected:expr) => {
        println!("{:?}", $n.to_le_bytes());
        assert_eq!($expected, $n.to_le_bytes());
    }
}

fn main() {
    check!(1u8, [1]);
    check!(1u16, [1, 0]);
    check!(1u32, [1, 0, 0, 0]);
    check!(1u64, [1, 0, 0, 0, 0, 0, 0, 0]);
    check!(1u128, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}
