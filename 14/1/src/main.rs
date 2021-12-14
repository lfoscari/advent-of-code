use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines =  contents.lines();

    let mut polymer = lines.next().unwrap();
    let formulas: Vec<(&str, &str)> = lines.skip(1).map(|s| s.split_once(" -> ").unwrap()).collect();

    for formula in formulas {

    }

    println!("{}\n{:?}", template, formulas);
}
