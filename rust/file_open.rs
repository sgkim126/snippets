use std::fs::File;

fn main() {
    let path = "file_open.rs".to_string();
    let f = File::open(&path);
    println!("{:?}", f);
}
