use std::iter::{IntoIterator, Iterator};

struct S;

struct Iter(Option<String>);

impl Iterator for Iter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.0.clone() {
            self.0 = None;
            Some(value)
        } else {
            None
        }
    }
}

impl IntoIterator for S {
    type Item = String;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter(Some("move".to_string()))
    }
}

impl IntoIterator for &S {
    type Item = String;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter(Some("ref".to_string()))
    }
}

impl IntoIterator for &mut S {
    type Item = String;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter(Some("mut ref".to_string()))
    }
}

fn main() {
    for x in S {
        println!("{}", x);
    }
    for x in &S {
        println!("{}", x);
    }
    for x in &mut S {
        println!("{}", x);
    }
}
