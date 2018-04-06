struct S {
    i: i32,
}

impl ::std::ops::Drop for S {
    fn drop(&mut self) {
        println!("DROP {}", self.i);
    }
}

impl S {
    fn new(i: i32) -> Self {
        Self {
            i,
        }
    }
    fn get(&self) -> Option<&i32> {
        Some(&self.i)
    }
    fn get2(&self) -> Option<&i32> {
        None
    }
}

fn main() {
    println!("1");
    if let Some(_i) = S::new(1).get() {
        println!("if");
    } else {
        println!("else");
    }
    println!("2");
    if let Some(_i) = S::new(2).get2() {
        println!("if");
    } else {
        println!("else");
    }
    println!("3");
    if let Some(_) = S::new(3).get() {
        println!("if");
    } else {
        println!("else");
    }
    println!("4");
    if let Some(_) = S::new(4).get2() {
        println!("if");
    } else {
        println!("else");
    }
    println!("5");
}
