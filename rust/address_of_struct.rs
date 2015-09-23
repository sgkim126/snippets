struct EmptyStruct;

fn main() {
    let e1 = EmptyStruct;
    let e2 = EmptyStruct;
    println!("{:?}", &e1 as *const EmptyStruct);
    println!("{:?}", &e2 as *const EmptyStruct);

    assert!(&e1 as *const EmptyStruct != &e2 as *const EmptyStruct);
}
