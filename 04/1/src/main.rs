use std::fs;
use regex::Regex;

fn assign_point(number: i32, table: &Vec<Vec<i32>>, points: &mut Vec<Vec<i32>>) {
    let table_side = table.len();

    for i in 0..table_side {
        for j in 0..table_side {
            if table[i][j] == number {
                points[i][j] = 1;
            }
        }
    }
}

fn check_win(points: &Vec<Vec<i32>>) -> bool {
    let table_side = points.len();

    for i in 0..table_side {
        if points[i].iter().all(|&x| x == 1) {
            return true;
        }

        if points.iter().all(|x| x[i] == 1) {
            return true;
        }
    }

    false
}

fn sum_unmarked(table: &Vec<Vec<i32>>, points: &Vec<Vec<i32>>) -> i32 {
    let table_side = table.len();
    let mut score = 0;

    for i in 0..table_side {
        for j in 0..table_side {
            if points[i][j] == 0 {
                score += table[i][j];
            }
        }
    }

    score
}

fn parse_table(t: &str) -> Vec<Vec<i32>> {
    t.split("\n")
    .filter(|x|
        x.len() > 0)
    .map(|row|
        row.split_whitespace().map(|x|
            x.parse().unwrap()).collect())
    .collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let empty_line = Regex::new(r"\n\n").unwrap();

    let (first, rest) = contents.split_once("\n").unwrap();

    let extractions = first.split(",").map(|x| x.parse::<i32>().unwrap());

    let tables: Vec<_> = empty_line.split(rest).map(|t| parse_table(t)).collect();
    let tables_amount = tables.len();

    let mut selected: Vec<Vec<Vec<i32>>> = tables.iter().map(|table| table.iter().map(|row| row.iter().map(|_| 0).collect()).collect()).collect();

    for number in extractions {
        for i in 0..tables_amount {
            assign_point(number, &tables[i], &mut selected[i]);

            if check_win(&selected[i]) {
                let score = sum_unmarked(&tables[i], &selected[i]) * number;
                println!("Win with score {}", score);
                return;
            }
        }
    }

    println!("No one wins...");
}
