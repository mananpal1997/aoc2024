pub fn solve(_input: &str) -> i32 {
    let lines: Vec<&str> = _input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let deltas = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let target = "XMAS".as_bytes();
    let mut matches = 0;

    for i in 0..num_rows {
        for j in 0..num_cols {
            if lines[i].as_bytes()[j] != target[0] {
                continue;
            }

            for &(dx, dy) in &deltas {
                let mut valid = 1;
                for k in 1..target.len() {
                    let nx = i as isize + k as isize * dx;
                    let ny = j as isize + k as isize * dy;
                    if nx < 0 || ny < 0 || nx >= num_rows as isize || ny >= num_cols as isize {
                        valid = 0;
                        break;
                    }
                    if lines[nx as usize].as_bytes()[ny as usize] != target[k] {
                        valid = 0;
                        break;
                    }
                }
                matches += valid;
            }
        }
    }

    matches
}