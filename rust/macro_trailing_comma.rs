macro_rules! force_trailing_comma {
    [$($x:expr,)*] => (vec![$($x),*])
}

macro_rules! forbid_trailing_comma {
    [$($x:expr),*] => (vec![$($x),*])
}

macro_rules! allow_both {
    [$($x:expr,)*] => { force_trailing_comma![$($x,)*] };
    [$($x:expr),*] => { forbid_trailing_comma![$($x),*] };
}

fn main() {
    force_trailing_comma![1,];
    // force_trailing_comma![1]; // compile error
    // forbid_trailing_comma![1,]; // compile error
    forbid_trailing_comma![1];
    allow_both![1,];
    allow_both![1];
}
