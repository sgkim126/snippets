fn main() {
    let a = 0xffff_ffffu32;
    let b = a as i32;
    let c = b as i64;
    println!("{} {} {}", a, b, c); // 4294967295 -1 -1
}
