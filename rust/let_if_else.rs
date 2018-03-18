use std::ops::Drop;

struct A;
impl Drop for A {
    fn drop(&mut self) {
        println!("drop");
    }
}

enum E {
    A(A),
    B(A),
}

fn main() {
    println!("start");
    if let E::B(a) = E::A(A{}) {
        println!("if");
    } else {
        println!("else");
    }
    println!("fin");
}
