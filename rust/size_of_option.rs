use std::mem::size_of;

fn main() {
    println!("{}", size_of::<u32>()); // 4
    println!("{}", size_of::<u64>()); // 8
    println!("{}", size_of::<Option<u32>>()); // 8
    println!("{}", size_of::<Option<u64>>()); // 16

    assert_eq!(size_of::<u32>() * 2, size_of::<u64>());
}
