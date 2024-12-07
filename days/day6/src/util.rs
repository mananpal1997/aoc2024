use std::collections::HashSet;

pub fn get_covered_cells(lines: &Vec<Vec<u8>>, start_pos: (isize, isize)) -> (HashSet<(isize, isize)>, bool) {
    let num_rows = lines.len() as isize;
    let num_cols = lines.iter().map(|line| line.len()).max().unwrap_or(0) as isize;
    
    let mut covered: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
    let mut direction: (isize, isize) = (-1, 0);
    let mut guard_pos = start_pos;

    let mut is_loop = true;

    while !covered.contains(&(guard_pos, direction)) {
        covered.insert((guard_pos, direction));
        let new_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        let out_of_bounds = new_pos.0 < 0 || new_pos.0 >= num_rows || new_pos.1 < 0 || new_pos.1 >= num_cols;
        if out_of_bounds {
            is_loop = false;
            break
        }
        if lines[new_pos.0 as usize][new_pos.1 as usize] == b'#' {
            match direction {
                (0, -1) => direction = (-1, 0),
                (1, 0) => direction = (0, -1),
                (0, 1) => direction = (1, 0),
                (-1, 0) => direction = (0, 1),
                _ => {}
            }
        } else {
            guard_pos = new_pos;
        }
    }

    let cells: HashSet<(isize, isize)> = covered.iter().map(|(cell, _)| *cell).collect();
    (cells, is_loop)
}

pub fn get_guard_pos(lines: &Vec<Vec<u8>>) -> (isize, isize) {
    lines
    .iter()
    .enumerate()
    .find_map(|(row_idx, line)| {
        line
        .iter()
        .enumerate()
        .find_map(|(col_idx, &ch)| {
            if ch == b'^' {
                Some((row_idx as isize, col_idx as isize))
            } else {
                None
            }
        })
    }).unwrap_or((-1, -1))
}