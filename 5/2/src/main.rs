use std::collections::HashMap;
use std::fs;
use std::cmp;

fn parse_xy(s: &str) -> (i32, i32) {
    let (x, y) = s.split_once(",").expect("Coordinates not well formatted");
    (x.parse().unwrap(), y.parse().unwrap())
}

fn coordinates_tuple(s: &str) -> ((i32, i32), (i32, i32)) {
    let (from, to) = s.split_once(" -> ").expect("Coordinates ranges not well formatted");
    (parse_xy(from), parse_xy(to))
}

// m = (fy - ty) / (fx - tx) = 1 (diagonal), 0 (horizontal), NaN (vertical)
fn interlap_points(((fx, fy), (tx, ty)): ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    match (fy - ty).checked_div(fx - tx) {
        Some(d) if d.abs() == 1 => {
            let mut spanx: Vec<i32> = (cmp::min(fx, tx)..=cmp::max(fx, tx)).collect();
            let mut spany: Vec<i32> = (cmp::min(fy, ty)..=cmp::max(fy, ty)).collect();

            if spanx[0] != fx {
                spanx.reverse();
            }
            
            if spany[0] != fy {
                spany.reverse();
            }

            spanx.into_iter().zip(spany.into_iter()).collect()
        }

        Some(0) => (cmp::min(fx, tx)..=cmp::max(fx, tx)).map(|x| (x, fy)).collect(),
        None    => (cmp::min(fy, ty)..=cmp::max(fy, ty)).map(|y| (fx, y)).collect(),

        Some(n) => panic!("Coordinates don't follow the rules ({}): {:?} -> {:?}", n, (fx, fy), (tx, ty))
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut floor: HashMap<(i32, i32), i32> = HashMap::new();

    let coordinates = contents
        .lines()
        .map(coordinates_tuple)
        .flat_map(interlap_points);

    for (x, y) in coordinates {
        let count = floor.entry((x, y)).or_insert(0);
        *count += 1;
    }
    
    floor.retain(|_, o| o > &mut 1);
    println!("Overlapping: {}", floor.len());
}
