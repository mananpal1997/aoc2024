use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet}};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct State {
    row: isize,
    col: isize,
    dir_row: isize,
    dir_col: isize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row
            .cmp(&other.row)
            .then(self.col.cmp(&other.col))
            .then(self.dir_row.cmp(&other.dir_row))
            .then(self.dir_col.cmp(&other.dir_col))
    }
}

pub fn solve(_input: &str) -> u64 {
    let mut free_cells: HashSet<(isize, isize)> = HashSet::new();
    let lines: Vec<&str> = _input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let (mut start_r, mut start_c) = (0, 0);
    let (mut end_r, mut end_c) = (0, 0);

    for r in 1..num_rows-1 {
        for c in 1..num_cols-1 {
            if lines[r].as_bytes()[c] == b'.' {
                free_cells.insert((r as isize, c as isize));
            }
            if lines[r].as_bytes()[c] == b'S' {
                free_cells.insert((r as isize, c as isize));
                (start_r, start_c) = (r as isize, c as isize);
            }
            if lines[r].as_bytes()[c] == b'E' {
                free_cells.insert((r as isize, c as isize));
                (end_r, end_c) = (r as isize, c as isize);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();

    let initial_state = State{row: start_r, col: start_c, dir_row: 0, dir_col: 1};
    heap.push(Reverse((0, initial_state)));
    costs.insert(initial_state, 0);
    
    while let Some(Reverse((cur_cost, state))) = heap.pop() {
        if state.row == end_r && state.col == end_c {
            return cur_cost;
        }

        if cur_cost > *costs.get(&state).unwrap_or(&u64::MAX) {
            continue;
        }

        let new_state = State{row: state.row + state.dir_row, col: state.col + state.dir_col, dir_row: state.dir_row, dir_col: state.dir_col};
        if free_cells.contains(&(new_state.row, new_state.col)) {
            let new_cost = cur_cost + 1;
            if new_cost < *costs.get(&new_state).unwrap_or(&u64::MAX) {
                costs.insert(new_state, new_cost);
                heap.push(Reverse((new_cost, new_state)));
            }
        }

        if state.dir_col == 0 {
            for new_dir_c in [-1, 1] {
                let new_state = State{row: state.row, col: state.col + new_dir_c, dir_row: 0, dir_col: new_dir_c};
                if !free_cells.contains(&(new_state.row, new_state.col)) {
                    continue;
                }
                let new_cost = cur_cost + 1001;
                if new_cost < *costs.get(&new_state).unwrap_or(&u64::MAX) {
                    costs.insert(new_state, new_cost);
                    heap.push(Reverse((new_cost, new_state)));
                }
            }
        }
        else if state.dir_row == 0 {
            for new_dir_r in [-1, 1] {
                let new_state = State{row: state.row + new_dir_r, col: state.col, dir_row: new_dir_r, dir_col: 0};
                if !free_cells.contains(&(new_state.row, new_state.col)) {
                    continue;
                }
                let new_cost = cur_cost + 1001;
                if new_cost < *costs.get(&new_state).unwrap_or(&u64::MAX) {
                    costs.insert(new_state, new_cost);
                    heap.push(Reverse((new_cost, new_state)));
                }
            }
        }
    }
    u64::MAX
}