#![feature(catch_panic)]

use std::thread::catch_panic;

fn panic() {
    panic!("panic message {can} be formatted", can = "CaN");
}

fn main() {
    let result = catch_panic(panic);
    println!("Result: {:?}", result);
}
