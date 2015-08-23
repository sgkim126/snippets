macro_rules! add_a {
    ($x: expr) => {
        $x + a;
    }
}

fn main() {
    let a = 1;
    add_a!(2);
}
