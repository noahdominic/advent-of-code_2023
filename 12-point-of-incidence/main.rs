use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input.txt")?);

    let lines = reader
        .lines()
        .filter_map(|line| line.ok().map(|l| l.trim().chars().collect()))
        .collect::<Vec<Vec<char>>>();

    let grids: Vec<&[Vec<char>]> = lines.split(|inner_vec| inner_vec.is_empty()).collect();

    println!("{}", grids.len());

    let mut total = 0;

    for grid in &grids {
        let cols: Vec<String> = (0..grid[0].len())
            .map(|i| grid.iter().map(|row| row[i]).collect())
            .collect();
        let rows: Vec<String> = grid.iter().map(|x| x.iter().collect()).collect();

        if let Some(col_symm) = find_axis_of_symmetry(&cols) {
            total += col_symm;
        } else if let Some(row_symm) = find_axis_of_symmetry(&rows) {
            total += row_symm * 100;
        } else {
            assert!(true);
        }
    }
    println!("{}", total);

    println!();

    total = 0;

    for grid in &grids {
        let cols: Vec<String> = (0..grid[0].len())
            .map(|i| grid.iter().map(|row| row[i]).collect())
            .collect();
        let rows: Vec<String> = grid.iter().map(|x| x.iter().collect()).collect();

        if let Some(row_symm) = fuzzy_find_axis_of_symmetry(&rows) {
            // println!("Axis row symm: {:?}", row_symm);

            total += row_symm * 100;
        } else if let Some(col_symm) = fuzzy_find_axis_of_symmetry(&cols) {
            // println!("Axis col symm: {:?}", col_symm);

            total += col_symm;
        } else {
            assert!(true);
        }
    }
    println!("{}", total);

    // total = 0;

    // for (i, grid) in grids.iter().enumerate() {
    //     let cols: Vec<usize> = (0..grid[0].len())
    //         .map(|i| binary_to_dec(&grid.iter().map(|row| row[i]).collect()))
    //         .collect();
    //     let rows: Vec<usize> = grid.iter().map(|x| binary_to_dec(x)).collect();
    //     if let Some(col_symm) = find_fuzzy_axis_of_symmetry(&cols, 1, prev_answers[i]) {
    //         println!("Axis col symm: {:?}", col_symm);
    //         total += col_symm;
    //     } else if let Some(row_symm) = find_fuzzy_axis_of_symmetry(&rows, 2, prev_answers[i]) {
    //         println!("Axis row symm: {:?}", row_symm);
    //         total += row_symm * 100;
    //     } else {
    //         // return Err("FUCK".into());
    //         println!("{}", i);
    //     }

    //     println!();

    //     println!("{}", total);
    // }

    Ok(())
}

// fn find_fuzzy_axis_of_symmetry(
//     vector: &Vec<usize>,
//     is_col: usize,
//     prev_answer: (usize, usize),
// ) -> Option<usize> {
//     let length = if vector.len() == 9 { 7 } else { 9 };

//     for item_idx in 0..vector.len() {
//         let mut temp_vec = vector.clone();
//         let original = vector[item_idx];
//         for bit_idx in 0..length {
//             if let Some(element) = temp_vec.get_mut(item_idx) {
//                 *element = original;
//                 *element = flip_bit(*element, bit_idx);
//             }
//             if let Some(axis) = find_axis_of_symmetry(&temp_vec) {
//                 if !(prev_answer.1 == axis && prev_answer.0 == is_col) {
//                     println!("New axis is {}", axis);
//                     return Some(axis);
//                 }
//             }
//         }
//     }

//     None
// }

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
        let mut i = idx;
        let mut j = idx - 1;
        let mut acc = 0;

        while i < length && fuzzycmp(vector[i].as_str(), vector[j].as_str(), &mut fuzzy_flag) {
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

fn fuzzycmp(s1: &str, s2: &str, fuzzy_flag: &mut bool) -> bool {
    let diff_count = s1
        .chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();

    if !*fuzzy_flag || s1 == s2 {
        return s1 == s2;
    }

    if diff_count == 1 {
        *fuzzy_flag = false;
        return true;
    }

    return s1 == s2;
}
