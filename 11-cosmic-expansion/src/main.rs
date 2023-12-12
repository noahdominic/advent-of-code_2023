use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input.txt")?);

    let grid = reader
        .lines()
        .filter_map(|line| line.ok().map(|l| l.trim().chars().collect()))
        .collect::<Vec<Vec<char>>>();

    let galaxy_coords = get_expanded_galaxy_coords(&grid, 1000000 - 1);

    let mut total_distance = 0;

    for i in 0..galaxy_coords.len() {
        for j in i..galaxy_coords.len() {
            total_distance += get_manhattan_distance(galaxy_coords[i], galaxy_coords[j]);
        }
    }

    println!("{:?}", total_distance);

    Ok(())
}

fn get_expanded_galaxy_coords(
    grid: &Vec<Vec<char>>,
    expansion_factor: usize,
) -> Vec<(usize, usize)> {
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

    // expand galaxy coords here
    for i in 0..galaxies.len() {
        let expansion_col = empty_cols
            .iter()
            .filter(|&&col| col < galaxies[i].1)
            .collect::<Vec<&usize>>()
            .len()
            * expansion_factor;
        let expansion_row = empty_rows
            .iter()
            .filter(|&&row| row < galaxies[i].0)
            .collect::<Vec<&usize>>()
            .len()
            * expansion_factor;
        galaxies[i] = (galaxies[i].0 + expansion_row, galaxies[i].1 + expansion_col)
    }

    return galaxies;
}

fn get_manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let (row1, col1) = p1;
    let (row2, col2) = p2;

    let dcol = col2.abs_diff(col1);
    let drow = row2.abs_diff(row1);

    dcol + drow
}
