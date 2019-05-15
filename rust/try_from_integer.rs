use std::convert::TryFrom;

fn main() {
    println!("{:?}", std::usize::MAX);
    println!("{:?}", i32::try_from(std::usize::MAX));
    println!("{:?}", std::usize::MAX as i32);
    println!("{:?}", u32::try_from(std::usize::MAX));
    println!("{:?}", std::usize::MAX as u32);
}
