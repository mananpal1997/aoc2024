use std::{collections::{HashMap, HashSet, VecDeque}, i32};

pub fn solve(_input: &str) -> &str {
    let lines: Vec<_> = _input.lines().collect();

    let (rows, cols) = lines[0].split_once(",").map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())).unwrap();
    let bytes_to_fell = lines[1].parse::<i32>().unwrap();

    let (mut l, mut r) = (1, bytes_to_fell);
    while l <= r {
        let m = (r + l) / 2;
        let corruptions: HashSet<(i32, i32)> = lines.iter().skip(2).take(m as usize).map(|line| line.split_once(",").map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())).unwrap()).collect();
        let mut costs: HashMap<(i32, i32), i32> = HashMap::from([((0, 0), 0)]);
        let mut transitions = VecDeque::from([(0, 0)]);
        while !transitions.is_empty() {
            let (curr_r, curr_c) = transitions.pop_front().unwrap();
            if curr_r == rows - 1 && curr_c == cols - 1 {
                break;
            }
            for (dr, dc) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let (next_r, next_c) = (curr_r + dr, curr_c + dc);
                if next_r < 0 || next_r >= rows || next_c < 0 || next_c >= cols {
                    continue;
                }
                if corruptions.contains(&(next_r, next_c)) {
                    continue;
                }
                let next_cost = costs.get(&(curr_r, curr_c)).unwrap() + 1;
                if next_cost < *costs.get(&(next_r, next_c)).unwrap_or(&i32::MAX) {
                    costs.insert((next_r, next_c), next_cost);
                    transitions.push_back((next_r, next_c));
                }
            }
        }
        if *costs.get(&(rows - 1, cols - 1)).unwrap_or(&i32::MAX) == i32::MAX {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }
    lines[(l + 1) as usize]
}