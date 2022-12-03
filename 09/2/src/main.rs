use std::fs;

fn basin_size(heatmap: &Vec<Vec<i32>>, basin: &mut Vec<Vec<bool>>, i: usize, j: usize, size: &mut i32) -> i32 {
    let h = heatmap.len();
    let w = heatmap[0].len();

    if basin[i][j] || heatmap[i][j] == 9 {
        return *size
    }

    basin[i][j] = true;
    
    if i > 0 && !basin[i - 1][j] {
        *size = basin_size(heatmap, basin, i - 1, j, size);
    }

    if j > 0 && !basin[i][j - 1] {
        *size = basin_size(heatmap, basin, i, j - 1, size);
    }

    if i < h - 1 && !basin[i + 1][j] {
        *size = basin_size(heatmap, basin, i + 1, j, size);
    }

    if j < w - 1 && !basin[i][j + 1] {
        *size = basin_size(heatmap, basin, i, j + 1, size);
    }

    *size + 1
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let heatmap: Vec<Vec<i32>> = contents
            .split("\n")
            .map(|s|
                s.split("")
                .filter(|d| !d.is_empty())
                .map(|d| d.parse().unwrap())
                .collect()
            )
            .collect();

    let h = heatmap.len();
    let w = heatmap[0].len();

    let mut basin_bitmap: Vec<Vec<bool>> = (0..h).map(|_| (0..w).map(|_| false).collect()).collect();
    let mut basins = vec![];

    for i in 0..h {
        for j in 0..w {
            if heatmap[i][j] == 9 || basin_bitmap[i][j] {
                continue;
            }
            basins.push(basin_size(&heatmap, &mut basin_bitmap, i, j, &mut 0));
        }
    }

    basins.sort_by(|a, b| b.cmp(a));
    let top_three = &basins[..3];

    println!("{:?}", top_three.iter().product::<i32>());
}
