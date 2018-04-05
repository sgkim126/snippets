struct S;

fn f() -> Vec<S> {
    vec![S, S, S]
}

fn main() {
    let a = f().first();

}
