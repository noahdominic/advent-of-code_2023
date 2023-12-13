use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input.txt")?);

    let lines = reader
        .lines()
        .filter_map(|line| line.ok().map(|l| l.trim().chars().collect()))
        .collect::<Vec<Vec<char>>>();

    let grids: Vec<&[Vec<char>]> = lines.split(|inner_vec| inner_vec.is_empty()).collect();

    let mut total = 0;
    let mut total2 = 0;

    for grid in &grids {
        let cols: Vec<String> = (0..grid[0].len())
            .map(|i| grid.iter().map(|row| row[i]).collect())
            .collect();
        let rows: Vec<String> = grid.iter().map(|x| x.iter().collect()).collect();
        if let Some(row_symm) = find_axis_of_symmetry(&rows) {
            total += row_symm * 100;
        } else if let Some(col_symm) = find_axis_of_symmetry(&cols) {
            total += col_symm;
        }

        if let Some(row_symm) = fuzzy_find_axis_of_symmetry(&rows) {
            total2 += row_symm * 100;
        } else if let Some(col_symm) = fuzzy_find_axis_of_symmetry(&cols) {
            total2 += col_symm;
        }
    }

    println!("{}", total);
    println!("{}", total2);

    Ok(())
}

fn find_axis_of_symmetry(vector: &Vec<String>) -> Option<usize> {
    let length = vector.len();

    for idx in 1..length {
        let mut i = idx;
        let mut j = idx - 1;
        let mut acc = 0;

        while i < length && vector[i] == vector[j] {
            acc += 1;
            if i == length - 1 {
                return Some(length - acc);
            }
            if j == 0 {
                return Some(acc);
            }
            j -= 1;
            i += 1;
        }
    }
    None
}

fn fuzzy_find_axis_of_symmetry(vector: &Vec<String>) -> Option<usize> {
    let length = vector.len();

    for idx in 1..length {
        let mut fuzzy_flag = true;
        let (mut i, mut j) = (idx, idx - 1);
        let mut acc = 0;

        while i < length && fuzzycmp(vector[i].as_str(), vector[j].as_str(), &mut fuzzy_flag) {
            acc += 1;
            if !fuzzy_flag {
                if i == length - 1 {
                    return Some(length - acc);
                } else if j == 0 {
                    return Some(acc);
                }
            } else {
                if i == length - 1 || j == 0 {
                    break;
                }
            }

            i += 1;
            j -= 1;
        }
    }
    None
}

fn fuzzycmp(s1: &str, s2: &str, fuzzy_flag: &mut bool) -> bool {
    if *fuzzy_flag == false {
        return s1 == s2;
    }

    let diff_count = s1
        .chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();

    if diff_count == 1 {
        *fuzzy_flag = false;
        return true;
    }

    s1 == s2
}
