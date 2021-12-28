use std::fs;

// fn find_path(from: (usize, usize), risks: &Vec<Vec<i32>>, path: &mut Vec<Vec<bool>>) {
//     let (i, j) = from;
//     path[i][j] = true;
//     match from {
//         (0, 0) => return,
//         (i, 0) => find_path((i - 1, 0), risks, path),
//         (0, j) => find_path((0, j - 1), risks, path),
//         (i, j) => {
//             if risks[i - 1][j] <= risks[i][j - 1] {
//                 find_path((i - 1, j), risks, path)
//             } else {
//                 find_path((i, j - 1), risks, path)
//             }
//         }
//     }
// }

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Input file not found");

    let grid: Vec<Vec<i32>> = contents
        .lines()
        .map(|l| l
            .split("")
            .filter(|&v| v != "")
            .map(|v| v.parse().unwrap())
            .collect()
        )
        .collect();

    let size = grid.len();

    // Dynamic programming

    // let mut risks: Vec<Vec<i32>> = (0..size).map(|_| (0..size).map(|_| 0).collect()).collect();

    // for i in 0..size {
    //     for j in 0..size {
    //         risks[i][j] = match (i, j) {
    //             (0, 0) => 0,
    //             (0, _) => grid[i][j] + risks[0][j - 1],
    //             (_, 0) => grid[i][j] + risks[i - 1][0],
    //             (_, _) => grid[i][j] + risks[i - 1][j].min(risks[i][j - 1])
    //         }
    //     }
    // }

    // let mut path: Vec<Vec<bool>> = (0..size).map(|_| (0..size).map(|_| false).collect()).collect();
    // find_path((size - 1, size - 1), &risks, &mut path);

    // for i in 0..size {
    //     for j in 0..size {
    //         if path[i][j] {
    //             print!("[{}]", grid[i][j]);
    //         } else {
    //             print!(" {} ", grid[i][j]);
    //         }
    //     }
    //     println!("");
    // }

    // println!("Risk: {:?}", risks[size - 1][size - 1]);

    // Dijkstra algorithm

    

    println!("{:?}", risks);
}