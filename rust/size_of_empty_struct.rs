use std::mem::size_of;

struct EmptyStruct;

fn main() {
    println!("{}", size_of::<EmptyStruct>()); // 0

    assert_eq!(size_of::<EmptyStruct>(), 0);
}
