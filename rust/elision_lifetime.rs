use std::slice::Iter;

fn f1<T>(a: &Vec<T>) -> Iter<T> {
    a.iter()
}

fn f2<'a, T>(a: &'a Vec<T>) -> Iter<'a, T> {
    a.iter()
}

fn main() {
    let x = {
        let v: Vec<i32> = vec![1, 2, 3, 4];
        f1(&v)
    };
    for k in x {
        println!("{}", k);
    }
    let x = {
        let v: Vec<i32> = vec![1, 2, 3, 4];
        f2(&v)
    };
    for k in x {
        println!("{}", k);
    }
}
