#![feature(drain_filter)]
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut fish: Vec<i32> = contents.split(",").map(|s| s.parse().unwrap()).collect();

    for _ in 0..80 {
        // println!("{:?}", fish);

        fish = fish.into_iter().map(|f| f - 1).collect();
        let newborns = fish.drain_filter(|&mut f| f < 0).collect::<Vec<i32>>().len();

        fish.append(&mut vec![8; newborns]);
        fish.append(&mut vec![6; newborns]);
    }

    println!("Total fish {}", fish.len());
}
