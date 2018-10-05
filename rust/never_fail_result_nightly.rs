#![feature(never_type)]

fn f() -> Result<usize, !> {
    Ok(0)
}

fn main() {
    println!("{}", f().unwrap());
}
