fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn walk_through_map(
    lr_seq: &str,
    map_dict: &std::collections::HashMap<String, [String; 2]>,
    start: &str,
) -> i64 {
    let mut current_loc = start.to_string();
    let lr_seq_len = lr_seq.len();
    let mut i = 0;

    while current_loc.chars().nth(2) != Some('Z') {
        let lr_index = if lr_seq.chars().nth(i % lr_seq_len) == Some('R') {
            1
        } else {
            0
        };
        current_loc = map_dict[&current_loc][lr_index].clone();
        i += 1;
    }

    i as i64
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let leftright_seq = lines[0].trim().to_string();
    let maps_data_str = &lines[2..];

    let maps_data_parsed: Vec<Vec<&str>> = maps_data_str
        .iter()
        .map(|line| {
            line.split(&[' ', '=', '(', ')', ','][..])
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect();

    let mut maps_dict: HashMap<String, [String; 2]> = HashMap::new();
    for map in maps_data_parsed {
        if map.len() >= 3 {
            maps_dict.insert(map[0].to_string(), [map[1].to_string(), map[2].to_string()]);
        }
    }

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();

    let mut answer: i64 = 1;

    for i in 0..26 {
        for j in 0..26 {
            let start_loc = format!("{}{}A", alphabet[i], alphabet[j]);
            if let Some(_) = maps_dict.get(&start_loc) {
                let num_steps = walk_through_map(&leftright_seq, &maps_dict, &start_loc);
                answer = answer * num_steps / gcd(answer, num_steps);
            }
        }
    }

    println!(
        "Part One: {}",
        walk_through_map(&leftright_seq, &maps_dict, "AAA")
    );
    println!("Part Two: {}", answer);

    Ok(())
}
