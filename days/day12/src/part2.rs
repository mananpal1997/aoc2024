use std::collections::HashSet;

const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn solve(_input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = _input.lines().map(|line| line.bytes().map(|b| b - b'A').collect()).collect();
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut ret = 0;
    for r in 0..num_rows {
        for c in 0..num_cols {
            if visited.contains(&(r, c)) {
                continue;
            }
            let mut area = 0;
            let mut sides = 0;
            let plot = grid[r as usize][c as usize];
            let mut stack: Vec<(isize, isize)> = Vec::from([(r, c)]);
            let mut edges: HashSet<(isize, isize, usize)> = HashSet::new();
            while !stack.is_empty() {
                let (curr_r, curr_c) = stack.pop().unwrap();
                if visited.contains(&(curr_r, curr_c)) {
                    continue;
                }
                visited.insert((curr_r, curr_c));
                area += 1;
                for (i, (dr, dc)) in DELTAS.iter().enumerate() {
                    let new_r = curr_r + dr;
                    let new_c = curr_c + dc;
                    if new_r < 0 || new_c < 0 || new_r >= num_rows || new_c >= num_cols || grid[new_r as usize][new_c as usize] != plot {
                        edges.insert((new_r, new_c, i));
                        sides += 1;
                        for (dr, dc) in DELTAS {
                            if edges.contains(&(new_r + dr, new_c + dc, i)) {
                                sides -= 1;
                            }
                        }
                        continue;
                    }
                    stack.push((new_r, new_c));
                }
            }
            ret += sides * area;
        }
    }

    ret
}