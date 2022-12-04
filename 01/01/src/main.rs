use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Input not found");

    let mut maximum = 0;
    let mut sum = 0;

    for line in input.lines() {
        if line == "" {
            maximum = cmp::max(maximum, sum);
            sum = 0;
        } else {
            let n: u32 = line.parse().unwrap();
            sum += n;
        }
    }

    println!("{:?}", maximum);
}
