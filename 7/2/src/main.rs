use std::fs;

fn error(xs: &Vec<i32>, center: i32) -> i32 {
    xs.iter().map(|&x| {
        let n = (x - center).abs();
        (n.pow(2) + n) / 2
    }).sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let positions: Vec<i32> = contents.split(",").map(|s| s.parse().unwrap()).collect();

    let (min, max) = (*positions.iter().min().unwrap(), *positions.iter().max().unwrap());
    let min_error = (min..=max).map(|center| error(&positions, center)).min().unwrap();

    println!("{:?}", min_error);
}