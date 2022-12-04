use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let mut count = 0;

    for assgns in input.lines() {
        let (frange, srange) = ranges(assgns);

        if frange.1 >= srange.0 && frange.0 <= srange.1 {
            count += 1;
        }
    }

    println!("{:?}", count);
}

fn ranges (assgns: &str) -> ((u32, u32), (u32, u32)) {
    let halfs: Vec<&str> = assgns.split(',').collect();
    return (minmax(halfs[0]), minmax(halfs[1]));
}

fn minmax (assgn: &str) -> (u32, u32) {
    let minmax: Vec<&str> = assgn.split('-').collect();
    return (minmax[0].parse().unwrap(), minmax[1].parse().unwrap());
}
