use std::fs;
use std::cmp;
use take_until::TakeUntilExt;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file");

    let grid: Vec<Vec<u32>> = input.lines()
        .map(|l| l.chars()
            .map(|c|
                c.to_digit(10).unwrap()
            ).collect()
        ).collect();

    let mut max_visibility = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            max_visibility = cmp::max(max_visibility, visibility(&grid, i, j));
        }
    }

    println!("{:?}", max_visibility);
}

fn visibility(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    (i + 1..grid.len()).take_until(|&_i| grid[_i][j] >= grid[i][j]).count() *
    (0..i).rev().take_until(|&_i| grid[_i][j] >= grid[i][j]).count() *
    (j + 1..grid[0].len()).take_until(|&_j| grid[i][_j] >= grid[i][j]).count() *
    (0..j).rev().take_until(|&_j| grid[i][_j] >= grid[i][j]).count()
}