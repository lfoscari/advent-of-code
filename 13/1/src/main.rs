#![feature(hash_drain_filter)]

use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut paper: HashSet<(i32, i32)> = HashSet::new();
    let (mut direction, mut split) = ("", 0);

    for line in (&contents).lines() {
        if line == "" {
            continue;
        } else if line.contains(",") {
            let (x, y) = line.split_once(",").unwrap();
            paper.insert((x.parse().unwrap(), y.parse().unwrap()));
        } else {
            let instruction = line.split(" ").last().unwrap().split_once("=").unwrap();
            direction = instruction.0;
            split = instruction.1.parse().unwrap();
            break;
        }
    }

    match direction {
        "x" => {
            let to_reposition = paper.drain_filter(|(x, _)| x > &split).collect::<Vec<_>>();
            for (x, y) in to_reposition {
                paper.insert((2 * split - x, y));
            }
        },
        "y" => {
            let to_reposition = paper.drain_filter(|(_, y)| y > &split).collect::<Vec<_>>();
            for (x, y) in to_reposition {
                paper.insert((x, 2 * split - y));
            }
        },
        _ => panic!("Unknown direction")
    }
    println!("{} points remaining", paper.len());
}