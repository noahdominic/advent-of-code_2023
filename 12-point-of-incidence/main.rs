use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input.txt")?);

    let lines = reader
        .lines()
        .filter_map(|line| line.ok().map(|l| l.trim().chars().collect()))
        .collect::<Vec<Vec<char>>>();

    let grids: Vec<&[Vec<char>]> = lines.split(|inner_vec| inner_vec.is_empty()).collect();

    // let test_row: Vec<usize> = vec![
    //     72, 79, 122, 122, 122, 122, 79, 72, 55, 52, 121, 124, 15, 76, 54, 74, 73,
    // ];

    // println!("{:?}", test_row);
    // println!("{:?}", find_axis_of_symmetry(&test_row));

    println!("{}", grids.len());

    let mut total = 0;

    // is_col, index of symmetry
    let mut prev_answers: Vec<(usize, usize)> = vec![];

    for grid in &grids {
        let cols: Vec<usize> = (0..grid[0].len())
            .map(|i| binary_to_dec(&grid.iter().map(|row| row[i]).collect()))
            .collect();
        let rows: Vec<usize> = grid.iter().map(|x| binary_to_dec(x)).collect();
        if let Some(col_symm) = find_axis_of_symmetry(&cols) {
            // println!("Axis col symm: {:?}", col_symm);

            total += col_symm;
            prev_answers.push((1, col_symm));
        } else if let Some(row_symm) = find_axis_of_symmetry(&rows) {
            // println!("Axis row symm: {:?}", row_symm);

            total += row_symm * 100;
            prev_answers.push((2, row_symm));
        } else {
            assert!(true);
            prev_answers.push((0, 0));
        }

        println!("{}", total);
    }

    println!();

    total = 0;

    for (i, grid) in grids.iter().enumerate() {
        let cols: Vec<usize> = (0..grid[0].len())
            .map(|i| binary_to_dec(&grid.iter().map(|row| row[i]).collect()))
            .collect();
        let rows: Vec<usize> = grid.iter().map(|x| binary_to_dec(x)).collect();
        if let Some(col_symm) = find_fuzzy_axis_of_symmetry(&cols, 1, prev_answers[i]) {
            println!("Axis col symm: {:?}", col_symm);
            total += col_symm;
        } else if let Some(row_symm) = find_fuzzy_axis_of_symmetry(&rows, 2, prev_answers[i]) {
            println!("Axis row symm: {:?}", row_symm);
            total += row_symm * 100;
        } else {
            // return Err("FUCK".into());
            println!("{}", i);
        }

        println!();

        println!("{}", total);
    }

    // for (i, grid) in grids.iter().enumerate() {
    //     let new_vec: Vec<usize> = if is_new_symm_in_col[i] {
    //         (0..grid[0].len())
    //             .map(|i| binary_to_dec(&grid.iter().map(|row| row[i]).collect()))
    //             .collect()
    //     } else {
    //         grid.iter().map(|x| binary_to_dec(x)).collect()
    //     };

    //     let length = if is_new_symm_in_col[i] { 7 } else { 9 };

    //     println!("{:?}", new_vec);
    //     let mut stop_now = false;

    //     for item_idx in 0..new_vec.len() {
    //         let mut temp_vec = new_vec.clone();
    //         let original = new_vec[item_idx];
    //         for bit_idx in 0..length {
    //             if let Some(element) = temp_vec.get_mut(item_idx) {
    //                 *element = original;
    //                 *element = flip_bit(*element, bit_idx);
    //             }
    //             if let Some(axis) = find_axis_of_symmetry(&temp_vec) {
    //                 println!("New axis is {}", axis);
    //                 stop_now = true;
    //                 break;
    //             }
    //         }
    //         if stop_now {
    //             break;
    //         }
    //     }
    // }

    Ok(())
}

fn find_fuzzy_axis_of_symmetry(
    vector: &Vec<usize>,
    is_col: usize,
    prev_answer: (usize, usize),
) -> Option<usize> {
    let length = if vector.len() == 9 { 7 } else { 9 };

    for item_idx in 0..vector.len() {
        let mut temp_vec = vector.clone();
        let original = vector[item_idx];
        for bit_idx in 0..length {
            if let Some(element) = temp_vec.get_mut(item_idx) {
                *element = original;
                *element = flip_bit(*element, bit_idx);
            }
            if let Some(axis) = find_axis_of_symmetry(&temp_vec) {
                if !(prev_answer.1 == axis && prev_answer.0 == is_col) {
                    println!("New axis is {}", axis);
                    return Some(axis);
                }
            }
        }
    }

    None
}

fn find_axis_of_symmetry(vector: &Vec<usize>) -> Option<usize> {
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

fn binary_to_dec(binary_vec: &Vec<char>) -> usize {
    let mut decimal_value = 0;

    for &c in binary_vec {
        decimal_value <<= 1; // Equivalent to decimal_value *= 2;
        match c {
            '.' => {} // Do nothing as it's already set by default (left shift operation)
            '#' => decimal_value |= 1, // Set the least significant bit to 1
            _ => return 0, // Invalid character
        }
    }

    decimal_value
}

fn flip_bit(num: usize, index: usize) -> usize {
    let mask = 1 << index;

    // Using XOR to flip the bit at the specified index
    num ^ mask
}
