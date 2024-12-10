use std::collections::HashSet;

const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn solve(_input: &str) -> usize {
    let grid: Vec<Vec<u8>> = _input.lines().map(|line| line.bytes().map(|b| b - b'0').collect()).collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let trailheads_count = |r: usize, c: usize| -> usize {
        let mut states: Vec<(usize, usize)> = Vec::from([(r, c)]);
        let mut trailheads: HashSet<(usize, usize)> = HashSet::new();
        while !states.is_empty() {
            let (curr_r, curr_c) = states.pop().unwrap();
            if grid[curr_r][curr_c] == 0 {
                trailheads.insert((curr_r, curr_c));
            }
            for (dr, dc) in DELTAS {
                if (curr_r == 0 && dr < 0) || (curr_c == 0 && dc < 0) || (curr_r == num_rows - 1 && dr > 0) || (curr_c == num_cols - 1 && dc > 0) {
                    continue;
                }
                let new_r = (curr_r as isize + dr) as usize;
                let new_c = (curr_c as isize + dc) as usize;
                if grid[curr_r][curr_c] != grid[new_r][new_c] + 1 {
                    continue;
                }
                states.push((new_r, new_c));
            }
        }
        trailheads.len()
    };

    grid.iter().enumerate().map(|(row_idx, row)| {
        let ret: usize = row.iter().enumerate().map(|(col_idx, col)| {
            match col {
                9 => trailheads_count(row_idx, col_idx),
                _ => 0
            }
        }).sum();
        ret
    }).sum()
}