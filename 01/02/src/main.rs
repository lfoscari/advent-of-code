use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Input not found");

    let mut totals = vec![];
    let mut sum = 0;

    for line in input.lines() {
        if line ==  "" {
            totals.push(sum);
            sum = 0;
        } else {
            let n: u32 = line.parse().unwrap();
            sum += n;
        }
    }

    totals.sort();
    totals.reverse();
    totals.truncate(3);
    let top: u32 = totals.iter().sum();
    
    println!("{:?}", top);
}
