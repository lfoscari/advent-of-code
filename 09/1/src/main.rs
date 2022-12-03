use std::fs;

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

    let mut lowest: Vec<i32> = vec![];

    for i in 0..h {
        for j in 0..w {
            let x = heatmap[i][j];
            println!("Considering {}, {}", i, j);

            if i > 0 && x >= heatmap[i - 1][j] {
                continue;
            }

            if j > 0 && x >= heatmap[i][j - 1] {
                continue;
            }

            if i < h - 1 && x >= heatmap[i + 1][j] {
                continue;
            }

            if j < w - 1 && x >= heatmap[i][j + 1] {
                continue;
            }

            lowest.push(x);
        }
    }

    println!("{:?}", lowest);
    let risk = lowest.iter().map(|l| l + 1).sum::<i32>();
    println!("Risk: {}", risk);
}
