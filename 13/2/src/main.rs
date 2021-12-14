#![feature(hash_drain_filter)]

use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut paper: HashSet<(i32, i32)> = HashSet::new();
    let mut instructions: Vec<(&str, i32)> = vec![];

    for line in (&contents).lines() {
        if line == "" {
            continue;
        } else if line.contains(",") {
            let (x, y) = line.split_once(",").unwrap();
            paper.insert((x.parse().unwrap(), y.parse().unwrap()));
        } else {
            let direction = line.split(" ").last().unwrap().split_once("=").unwrap();
            instructions.push((direction.0, direction.1.parse().unwrap()));
        }
    }

    for (direction, split) in instructions.iter() {
        match *direction {
            "x" => {
                let to_reposition = paper.drain_filter(|(x, _)| x > split).collect::<Vec<_>>();
                for (x, y) in to_reposition {
                    paper.insert((2 * split - x, y));
                }
            },
            "y" => {
                let to_reposition = paper.drain_filter(|(_, y)| y > split).collect::<Vec<_>>();
                for (x, y) in to_reposition {
                    paper.insert((x, 2 * split - y));
                }
            },
            _ => panic!("Unknown direction")
        }
    }

    let paper_height = paper.iter().max_by(|(x, _), (_x, _)| x.cmp(_x)).unwrap().0 + 1;
    let paper_width = paper.iter().max_by(|(_, y), (_, _y)| y.cmp(_y)).unwrap().1 + 1;

    let mut map: Vec<Vec<bool>> = (0..paper_width).map(|_| (0..paper_height).map(|_| false).collect()).collect();
    for (x, y) in paper {
        map[y as usize][x as usize] = true;
    }

    for i in 0..paper_width {
        for j in 0..paper_height {
            if map[i as usize][j as usize] {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}