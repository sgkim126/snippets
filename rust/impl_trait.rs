trait T {
}

struct S0 {
}
struct S1 {
}
struct S2 {
}

impl T for S0 {
}
impl T for S1 {
}
impl T for S2 {
}

fn a(_a: Vec<impl T>) {
}

fn main() {
    a(vec![S0{}]);
    a(vec![S1{}]);
    a(vec![S2{}]);
    a(vec![S0{}, S1{}, S2{}]);
}
