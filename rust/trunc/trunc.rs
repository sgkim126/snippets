fn main() {
    let a = 3.1f32;
    let b = 3.9f32;
    let c = -3.1f32;
    let d = -3.9f32;

    assert_eq!(a.trunc(), 3.0);
    assert_eq!(b.trunc(), 3.0);
    assert_eq!(c.trunc(), -3.0);
    assert_eq!(d.trunc(), -3.0);
}

