#[derive(Debug)]
enum Never { }

fn f() -> Result<usize, Never> {
    Ok(0)
}

fn main() {
    println!("{}", f().unwrap());
}
