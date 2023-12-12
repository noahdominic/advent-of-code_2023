use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input.txt")?);

    let mut grid = reader
        .lines()
        .filter_map(|line| line.ok().map(|l| l.trim().chars().collect()))
        .collect::<Vec<Vec<char>>>();
    let mut grid2 = grid.clone();

    if let Some((s_row, s_col)) = find_start(&grid) {
        // BFS on the graph to get farthest
        let mut numsteps = 0;
        let mut search_space: VecDeque<(usize, usize)> = VecDeque::new();
        let mut search_buffer: VecDeque<(usize, usize)> = VecDeque::new();

        search_buffer.push_back((s_row, s_col));
        while search_buffer.len() > 0 {
            search_space.append(&mut search_buffer);
            while search_space.len() > 0 {
                if let Some((i, j)) = search_space.pop_front() {
                    search_buffer.append(&mut traverse_loop(&mut grid, i, j));
                }
            }
            if search_buffer.len() > 0 {
                numsteps += 1;
            }
        }
        println!("Numsteps: {}", numsteps);

        // DFS on graph to get vertices in order to get: perimeter & area
        //
        // Area is calculated via 'surveyor's formula'
        //
        // Internal points calculated via Pick's theorem (rearranged):
        //   internal_vtx_count = area  +  0.5 * perimeter  +  1

        let mut vertices: Vec<(usize, usize)> = vec![];
        search_buffer.push_back((s_row, s_col));

        while search_buffer.len() > 0 {
            if let Some((i, j)) = search_buffer.pop_back() {
                vertices.push((i, j));
                let mut a = traverse_loop(&mut grid2, i, j);
                search_buffer.append(&mut a);
            }
        }

        println!(
            "Internal vertices: {}",
            surveyors_formula(&vertices) + 1. - vertices.len() as f64 / 2.
        );
    }

    Ok(())
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }

    None
}

fn get_valid_directions(c: char) -> Vec<(isize, isize)> {
    match c {
        'S' => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
        '═' => vec![(0, 1), (0, -1)],
        '║' => vec![(1, 0), (-1, 0)],
        '╔' => vec![(0, 1), (1, 0)],
        '╗' => vec![(0, -1), (1, 0)],
        '╚' => vec![(0, 1), (-1, 0)],
        '╝' => vec![(0, -1), (-1, 0)],
        _ => vec![],
    }
}

fn get_replacement_char(c: char) -> char {
    match c {
        '-' => '═',
        '|' => '║',
        'F' => '╔',
        '7' => '╗',
        'L' => '╚',
        'J' => '╝',
        _ => c,
    }
}

fn traverse_loop(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> VecDeque<(usize, usize)> {
    let directions = get_valid_directions(grid[i][j]);

    let valid_neighbours: VecDeque<(usize, usize)> = directions
        .iter()
        .filter_map(|&(di, dj)| {
            let new_i = (i as isize + di) as usize;
            let new_j = (j as isize + dj) as usize;

            if new_i < grid.len() && new_j < grid[0].len() {
                match (di, dj) {
                    // Right
                    (0, 1) => match grid[new_i][new_j] {
                        '-' | '7' | 'J' => Some((new_i, new_j)),
                        _ => None,
                    },
                    // Down
                    (1, 0) => match grid[new_i][new_j] {
                        '|' | 'J' | 'L' => Some((new_i, new_j)),
                        _ => None,
                    },
                    // Left
                    (0, -1) => match grid[new_i][new_j] {
                        '-' | 'L' | 'F' => Some((new_i, new_j)),
                        _ => None,
                    },
                    // Up
                    (-1, 0) => match grid[new_i][new_j] {
                        '|' | '7' | 'F' => Some((new_i, new_j)),
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect();

    for (i, j) in &valid_neighbours {
        grid[*i][*j] = get_replacement_char(grid[*i][*j]);
    }

    return valid_neighbours;
}

fn surveyors_formula(vertices: &[(usize, usize)]) -> f64 {
    let mut area = 0.0;

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += (vertices[i].0 * vertices[j].1) as f64 - (vertices[j].0 * vertices[i].1) as f64;
    }

    area.abs() / 2.0
}
