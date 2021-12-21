use std::fs;
use std::collections::HashMap;

// I don't know why but I couldn't solve this very easy
// problem without writing this disgusting piece of code.

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines =  contents.lines();

    let mut polymer: Vec<char> = lines.next().unwrap().chars().collect();
    let mut formulas: HashMap<(char, char), char> = HashMap::new();
    
    // I'm sorry
    for line in lines.skip(1) {
        let (k, v) = line.split_once(" -> ").unwrap();
        let mut _k = k.chars();
        formulas.insert((_k.next().unwrap(), _k.next().unwrap()), v.chars().next().unwrap());
    }

    for _ in 0..10 {
        let mut new_polymer = polymer.clone();
        let mut added = 0;
        for i in 0..polymer.len() - 1 {
            match formulas.get(&(polymer[i], polymer[i+1])) {
                None => (),
                Some(&c) => {
                    added += 1;
                    new_polymer.insert(added + i, c)
                }
            }
        }
        polymer = new_polymer;
    }

    let mut elements = polymer.clone();
    elements.sort();
    elements.dedup();

    let counts: Vec<i32> = elements.iter().map(|e| polymer.iter().filter(|&a| a == e).count() as i32).collect();
    let max = counts.iter().max().unwrap();
    let min = counts.iter().min().unwrap();

    println!("Difference: {:?}", max - min);
}
