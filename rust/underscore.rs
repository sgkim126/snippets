use std::ops::Drop;

struct A;
impl Drop for A {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn main() {
    println!("1");
    A;
    println!("2");
    let a = A;
    println!("3");
    let _ = A;
    println!("4");
    let _a = A;
    println!("5");
}
