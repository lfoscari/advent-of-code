use std::collections::HashMap;
use std::fs;
use std::cmp;

fn parse_xy(s: &str) -> (i32, i32) {
    let (x, y) = s.split_once(",").expect("Coordinates not well formatted");
    (x.parse().unwrap(), y.parse().unwrap())
}

fn coordinates_tuple(s: &str) -> Option<((i32, i32), (i32, i32))> {
    let (from, to) = s.split_once(" -> ").expect("Coordinates ranges not well formatted");
    let ((x, y), (_x, _y)) = (parse_xy(from), parse_xy(to));

    if !(x == _x) && !(y == _y) {
        return None
    }

    Some(((x, y), (_x, _y)))
}

fn from_to_points(((fx, fy), (tx, ty)): ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    if fx == tx {
        let (min, max) = (cmp::min(fy, ty), cmp::max(fy, ty));
        return (min..=max).scan((fx, min), |_, cur| {
            Some((fx, cur))
        }).collect();
    } else {
        let (min, max) = (cmp::min(fx, tx), cmp::max(fx, tx));
        return (min..=max).scan((min, fy), |_, cur| {
            Some((cur, fy))
        }).collect();
    }
}

fn main() {
    
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut floor: HashMap<(i32, i32), i32> = HashMap::new();

    let coordinates = contents
        .lines()
        .filter_map(coordinates_tuple)
        .flat_map(from_to_points);

    for (x, y) in coordinates {
        let count = floor.entry((x, y)).or_insert(0);
        *count += 1;
    }

    let mut count = 0;
    for (_, overlapping) in floor {
        if overlapping > 1 {
            count += 1;
        }
    }

    println!("Overlapping: {}", count);
}
