use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input.txt")?);

    let grid = reader
        .lines()
        .filter_map(|line| line.ok().map(|l| l.trim().chars().collect()))
        .collect::<Vec<Vec<char>>>();

    let (galaxy_coords, (empty_rows, empty_cols)) = get_empty_rows_and_cols(&grid);

    let mut total_distance = 0;

    for i in 0..galaxy_coords.len() {
        for j in i..galaxy_coords.len() {
            let distance = get_expanded_manhattan_distance(
                galaxy_coords[i],
                galaxy_coords[j],
                &empty_rows,
                &empty_cols,
                1000000 - 1,
            );

            // println!(
            //     "{:?} to {:?} is distance {:?}",
            //     galaxy_coords[i], galaxy_coords[j], distance
            // );

            total_distance += distance;
        }
        // println!();
    }

    println!("{:?}", total_distance);

    // println!("{:?}", galaxy_coords);
    Ok(())
}

fn get_empty_rows_and_cols(
    grid: &Vec<Vec<char>>,
) -> (Vec<(usize, usize)>, (HashSet<usize>, HashSet<usize>)) {
    let mut empty_rows: HashSet<usize> = (0..grid.len()).collect();
    let mut empty_cols: HashSet<usize> = (0..grid[0].len()).collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                galaxies.push((i, j));
                empty_rows.remove(&i);
                empty_cols.remove(&j);
            }
        }
    }

    return (galaxies, (empty_rows, empty_cols));
}

fn get_manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let (row1, col1) = p1;
    let (row2, col2) = p2;

    let dcol = col2.abs_diff(col1);
    let drow = row2.abs_diff(row1);

    dcol + drow
}

fn get_expanded_manhattan_distance(
    p1: (usize, usize),
    p2: (usize, usize),
    empty_rows: &HashSet<usize>,
    empty_cols: &HashSet<usize>,
    expansion_factor: usize,
) -> usize {
    let (row1, col1) = p1;
    let (row2, col2) = p2;

    let (col1, col2) = if col1 < col2 {
        (col1, col2)
    } else {
        (col2, col1)
    };

    let (row1, row2) = if row1 < row2 {
        (row1, row2)
    } else {
        (row2, row1)
    };

    let expansion_col = empty_cols
        .iter()
        .filter(|&&col| col > col1 && col < col2)
        .collect::<Vec<&usize>>()
        .len()
        * expansion_factor;
    let expansion_row = empty_rows
        .iter()
        .filter(|&&row| row > row1 && row < row2)
        .collect::<Vec<&usize>>()
        .len()
        * expansion_factor;

    let dcol = col2.abs_diff(col1) + expansion_col;
    let drow = row2.abs_diff(row1) + expansion_row;

    dcol + drow
}
