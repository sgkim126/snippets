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

fn a(_: impl T) -> impl T {
    S0{}
}

fn main() {
    a(S0{});
    a(S1{});
    a(S2{});
}
