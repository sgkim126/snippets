fn main() {
    let _ = 1;
    println!("{}", _);
}

// $rustc ./reserved_identifier.rs
// error: expected expression, found reserved identifier `_`
//  --> ./reserved_identifier.rs:3:20
//   |
// 3 |     println!("{}", _);
//   |                    ^ expected expression
//
// error: aborting due to previous error
