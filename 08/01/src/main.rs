use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file");

    let grid: Vec<Vec<u32>> = input.lines()
        .map(|l| l.chars()
            .map(|c|
                c.to_digit(10).unwrap()
            ).collect()
        ).collect();

    let mut visible = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if check_visibility(&grid, i, j) {
                visible += 1;
            }
        }
    }

    println!("{:?}", visible);
}

fn check_visibility(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    (i + 1..grid.len()).all(|_i| grid[_i][j] < grid[i][j]) ||
    (0..i).all(|_i| grid[_i][j] < grid[i][j]) ||
    (j + 1..grid[0].len()).all(|_j| grid[i][_j] < grid[i][j]) ||
    (0..j).all(|_j| grid[i][_j] < grid[i][j])
}