use std::collections::HashMap;

pub fn solve(_input: &str) -> i32 {
    let mut frequency_cells: HashMap<u8, Vec<(isize, isize)>> = HashMap::new();
    let mut lines: Vec<Vec<u8>> = _input.lines().map(|line| line.bytes().collect()).collect();

    let num_rows = lines.len() as isize;
    let num_cols = lines.iter().map(|line| line.len()).max().unwrap_or(0) as isize;

    for i in 0..num_rows {
        for j in 0..num_cols {
            if lines[i as usize][j as usize] == b'.' {
                continue;
            }
            frequency_cells.entry(lines[i as usize][j as usize]).or_insert(Vec::new()).push((i, j));
        }
    }

    
    for (_, antennas) in frequency_cells {
        for i in 0..antennas.len() {
            for j in i+1..antennas.len() {
                let y_diff = antennas[i].0 - antennas[j].0;
                let x_diff = antennas[i].1 - antennas[j].1;

                let antinode1 = (antennas[i].0 + y_diff, antennas[i].1 + x_diff);
                if antinode1.0 >= 0 && antinode1.1 >= 0 && antinode1.0 < num_rows && antinode1.1 < num_cols {
                    lines[antinode1.0 as usize][antinode1.1 as usize] = b'#';
                }

                let antinode2 = (antennas[j].0 - y_diff, antennas[j].1 - x_diff);
                if antinode2.0 >= 0 && antinode2.1 >= 0 && antinode2.0 < num_rows && antinode2.1 < num_cols {
                    lines[antinode2.0 as usize][antinode2.1 as usize] = b'#';
                }
            }
        }
    }

    let mut antinodes = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if lines[i as usize][j as usize] == b'#' {
                antinodes += 1;
            }
        }
    }
    antinodes
}