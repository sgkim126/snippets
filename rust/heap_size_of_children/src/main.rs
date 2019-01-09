extern crate heapsize;

use heapsize::HeapSizeOf;

fn main() {
    let a: usize = 0;
    println!("{}", a.heap_size_of_children());
    let b: Box<usize> = a.into();
    println!("{}", b.heap_size_of_children());
    let c: Box<Box<usize>> = b.into();
    println!("{}", c.heap_size_of_children());
    let d: Vec<Box<Box<usize>>> = vec![c];
    println!("{}", d.heap_size_of_children());
}
