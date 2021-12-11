use std::fs;
use std::cmp;

fn flash(octos: &mut Vec<Vec<i32>>, i: usize, j: usize, h: usize, w: usize) {
    octos[i][j] = 0;

    for _i in cmp::max(i, 1)-1..=cmp::min(i+1, h-1) {
        for _j in cmp::max(j, 1)-1..=cmp::min(j+1, w-1) {
            if !(i == _i && j == _j) {
                if octos[_i][_j] > 0 {
                    octos[_i][_j] += 1;
                }
                if !(i == _i && j == _j) && octos[_i][_j] > 9 {
                    flash(octos, _i, _j, h, w);
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut octos: Vec<Vec<i32>> = contents
        .split("\n")
        .map(|l| l
            .split("")
            .filter(|o| !o.is_empty())
            .map(|o| o.parse().unwrap())
            .collect()
        ).collect();
    
    let mut flashes = 0;

    let h = octos.len();
    let w = octos[0].len();

    for _ in 1..=100 {
        for i in 0..h {
            for j in 0..w {
                octos[i][j] += 1;
            }
        }
        
        for i in 0..h {
            for j in 0..w {
                if octos[i][j] == 10 {
                    flash(&mut octos, i, j, h, w);
                }
            }
        }

        
        for i in 0..h {
            for j in 0..w {
                if octos[i][j] == 0 {
                    flashes += 1;
                }
            }
        }
    }

    println!("Total flashes: {}", flashes);
}
