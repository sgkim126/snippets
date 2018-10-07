use std::mem::size_of;

enum Never { }
struct ZeroSize;

type NeverFailResult<T> = Result<T, Never>;
type ZeroSizeResult<T> = Result<T, ZeroSize>;

fn main() {
    println!("{}", size_of::<u32>()); // 4
    println!("{}", size_of::<NeverFailResult<u32>>()); // 4
    println!("{}", size_of::<Option<u32>>()); // 8
    println!("{}", size_of::<NeverFailResult<Option<u32>>>()); // 8

    println!("{}", size_of::<u32>()); // 4
    println!("{}", size_of::<ZeroSizeResult<u32>>()); // 8
    println!("{}", size_of::<Option<u32>>()); // 8
    println!("{}", size_of::<ZeroSizeResult<Option<u32>>>()); // 8
}
