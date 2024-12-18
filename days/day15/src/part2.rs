use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn solve(_input: &str) -> i32 {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    _input.lines().filter(|line| line.starts_with("#")).for_each(|line| {
        let mut row: Vec<u8> = Vec::new();
        for ch in line.bytes() {
            match ch {
                b'#' => {
                    row.push(b'#');
                    row.push(b'#');
                },
                b'O' => {
                    row.push(b'[');
                    row.push(b']');
                },
                b'@' => {
                    row.push(b'@');
                    row.push(b'.');
                }
                b'.' => {
                    row.push(b'.');
                    row.push(b'.');
                }
                _ => {}
            }
        }
        grid.push(row);
    });

    let movement: HashMap<u8, (isize, isize)> = HashMap::from([
        (b'<', (0, -1)),
        (b'^', (-1, 0)),
        (b'v', (1, 0)),
        (b'>', (0, 1)),
    ]);

    let mut instructions: Vec<u8> = Vec::new();
    _input.lines().filter(|line| line.starts_with("^") || line.starts_with(">") || line.starts_with("v") || line.starts_with("<")).for_each(|line| {
        for ch in line.trim().bytes() {
            instructions.push(ch);
        }
    });

    let g_rows = grid.len() as isize;
    let g_cols = grid[0].len() as isize;

    let (mut robot_r, mut robot_c) = (0..g_rows).cartesian_product(0..g_cols).find(|&(r, c)| grid[r as usize][c as usize] == b'@').unwrap();

    'outer: for instruction in instructions {
        match movement.get(&instruction) {
            Some((d_row, d_col)) => {
                let mut queue = VecDeque::from([(robot_r, robot_c)]);
                let mut visited = HashSet::new();
                while !queue.is_empty() {
                    let (curr_row, curr_col) = queue.pop_front().unwrap();
                    if !visited.insert((curr_row, curr_col)) {
                        continue;
                    }
                    let (next_row, next_col) = (curr_row + d_row, curr_col + d_col);
                    match grid[next_row as usize][next_col as usize] {
                        b'O' => queue.push_back((next_row, next_col)),
                        b'[' => {
                            queue.push_back((next_row, next_col));
                            queue.push_back((next_row, next_col + 1));
                        },
                        b']' => {
                            queue.push_back((next_row, next_col));
                            queue.push_back((next_row, next_col - 1));
                        }
                        b'#' => continue 'outer,
                        _ => continue
                    }
                }
                for &(box_row, box_col) in visited.iter().sorted_by_key(|&&(r, c)| (robot_r.abs_diff(r), robot_c.abs_diff(c))).rev() {
                    let (next_row, next_col) = (box_row + d_row, box_col + d_col);
                    grid[next_row as usize][next_col as usize] = grid[box_row as usize][box_col as usize];
                    grid[box_row as usize][box_col as usize] = b'.';
                }
                (robot_r, robot_c) = (robot_r + d_row, robot_c + d_col);
            },
            None => {}
        }
    }


    let mut ret = 0;
    for y in 0..g_rows {
        for x in 0..g_cols {
            if grid[y as usize][x as usize] == b'[' {
                ret += 100 * y + x;
            }
        }
    }
    ret as i32
}