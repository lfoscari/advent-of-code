use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let values: Vec<u8> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    let mut fish: HashMap<u8, i64> = HashMap::new();

    for age in values {
        let f = fish.entry(age).or_insert(0);
        *f += 1;
    }

    for _ in 0..256 {
        let mut new_fish = HashMap::new();
        
        for (age, amount) in fish {
            let new_age = if age == 0 { new_fish.insert(8, amount); 6 } else { age - 1 };
            let f = new_fish.entry(new_age).or_insert(0);
            *f += amount;
        }

        fish = new_fish;
    }

    println!("Total fish: {}", fish.into_values().sum::<i64>());
}