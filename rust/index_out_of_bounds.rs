fn main() {
    let a = [1.0, 2.0, 3.0];

    println!("{:?} {:?}", a[1], a[3]); // thread '<main>' panicked at 'index out of bounds: the len is 3 but the index is 3', main.rs:4
}
