extern crate minigrep;

use std::ops::Index;

fn main() {
    let mut y = vec![5, 6];
    let mut x = vec![1, 2, 3];
    println!("={:?}", y.get((-1.max(0))));
}

//EOF
