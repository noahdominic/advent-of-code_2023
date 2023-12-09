use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_next(sequence: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    while !sequence.iter().all(|&x| x == 0) {
        sum += sequence.last().unwrap_or(&0);
        *sequence = sequence
            .iter()
            .enumerate()
            .take(sequence.len() - 1)
            .map(|(i, val)| sequence[i + 1] - val)
            .collect();
    }
    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect();

    let sum_first = lines
        .iter()
        .map(|sequence| get_next(&mut sequence.clone()))
        .sum::<i32>();
    let sum_second = lines
        .iter()
        .map(|sequence| get_next(&mut sequence.clone().into_iter().rev().collect()))
        .sum::<i32>();

    println!("{}", sum_first);
    println!("{}", sum_second);

    Ok(())
}
