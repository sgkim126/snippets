use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    const FOO: AtomicUsize = AtomicUsize::new(0);
    assert_eq!(0, FOO.fetch_add(10, Ordering::SeqCst));
    assert_eq!(0, FOO.load(Ordering::SeqCst));
    FOO.store(10, Ordering::SeqCst);
    assert_eq!(0, FOO.load(Ordering::SeqCst));
}
