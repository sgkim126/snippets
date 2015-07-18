use std::marker::PhantomData;
use std::mem::size_of;

struct Something(i32, f32, String);

fn main() {
    assert_eq!(0, size_of::<PhantomData<i32>>());
    assert_eq!(0, size_of::<PhantomData<Something>>());
}
