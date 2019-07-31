extern crate minigrep;

fn main() {
    let mut y = 0;
    let mut x = vec![1, 2, 3, 5, 6];
    for i in 0..x.len() {
        y ^= (i + 1) ^ x[i];
        println!("{:?}", y);
    }
    println!("={:?}", y);
}

//EOF
