use std::mem::size_of;

#[allow(dead_code)]
struct S {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

fn main() {
    println!("{}", size_of::<u32>()); // 4
    println!("{}", size_of::<u64>()); // 8
    println!("{}", size_of::<Option<u32>>()); // 8
    println!("{}", size_of::<Option<u64>>()); // 16

    println!("{}", size_of::<S>()); // 32
    println!("{}", size_of::<Option<S>>()); // 40

    assert_eq!(size_of::<S>() + 8, size_of::<Option<S>>());

    println!("{}", size_of::<&S>()); // 8
    println!("{}", size_of::<Option<&S>>()); // 8
    println!("{}", size_of::<Box<S>>()); // 8
    println!("{}", size_of::<Option<Box<S>>>()); // 8

    assert_eq!(size_of::<&S>(), size_of::<Option<&S>>());
    assert_eq!(size_of::<Box<S>>(), size_of::<Option<Box<S>>>());
}
