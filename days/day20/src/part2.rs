use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(_input: &str) -> u64 {
    let grid_lines: Vec<_> = _input.lines().filter(|line| line.starts_with("#")).collect();
    let rows = grid_lines.len();
    let cols = grid_lines[0].len();

    let time_to_save = _input.lines().last().unwrap().parse::<u64>().unwrap();

    let (mut start_r, mut start_c) = (0, 0);
    let (mut end_r, mut end_c) = (0, 0);

    let mut walls: HashSet<(usize, usize)> = HashSet::new();

    for r in 1..rows-1 {
        for c in 1..cols-1 {
            let val = grid_lines[r].as_bytes()[c];
            if val == b'#' {
                walls.insert((r, c));
            } else if val == b'S' {
                (start_r, start_c) = (r, c);
            } else if val == b'E' {
                (end_r, end_c) = (r, c);
            }
        }
    }

    let mut queue = VecDeque::from([(start_r, start_c)]);
    let mut times = HashMap::from([((start_r, start_c), 0)]);

    let get_neighbors = |r: usize, c: usize| -> [(usize, usize); 4] {
        [
            (r - 1, c),
            (r + 1, c),
            (r, c - 1),
            (r, c + 1)
        ]
    };

    while let Some((cur_r, cur_c)) = queue.pop_front() {
        if cur_r == end_r && cur_c == end_c {
            break;
        }
        for (next_r, next_c) in get_neighbors(cur_r, cur_c) {
            if next_r == 0 || next_r == rows - 1 || next_c == 0 || next_c == cols - 1 || walls.contains(&(next_r, next_c)) {
                continue;
            }
            let new_distance = *times.get(&(cur_r, cur_c)).unwrap() + 1;
            if new_distance < *times.get(&(next_r, next_c)).unwrap_or(&u64::MAX) {
                times.insert((next_r, next_c), new_distance);
                queue.push_back((next_r, next_c));
            }
        }
    }

    let mut cheats = 0;
    for (p1, t1) in times.iter() {
        for (p2, t2) in times.iter() {
            let jumps = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);
            if jumps > 20 {
                continue;
            }
            if *t2 >= time_to_save + t1 + jumps as u64 {
                cheats += 1;
            }
        }
    }

    cheats
}