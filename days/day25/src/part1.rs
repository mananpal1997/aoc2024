use std::collections::HashMap;

pub fn solve(_input: &str) -> u64 {
    let mut locks: HashMap<Vec<u64>, u64> = HashMap::new();
    let mut keys: HashMap<Vec<u64>, u64> = HashMap::new();

    let mut valid_pairs = 0;

    _input.split("\n\n").for_each(|schematic| {
        let lines: Vec<&str> = schematic.lines().collect();
        let schematic_height = lines.len();
        let schematic_width = lines[0].len();
        if lines[0].chars().filter(|&c| c == '.').count() == schematic_width {
            // key
            let mut key: Vec<u64> = Vec::new();
            for c in 0..schematic_width {
                key.push(0);
                for r in 1..schematic_height {
                    if lines[schematic_height - r - 1].as_bytes()[c] == b'.' {
                        break;
                    }
                    key[c] += 1;
                }
            }
            *keys.entry(key).or_default() += 1;
        } else {
            // lock
            let mut lock: Vec<u64> = Vec::new();
            for c in 0..schematic_width {
                lock.push(0);
                for r in 1..schematic_height {
                    if lines[r].as_bytes()[c] == b'.' {
                        break;
                    }
                    lock[c] += 1;
                }
            }
            *locks.entry(lock).or_default() += 1;
        }
    });

    for (key, kf) in keys.iter() {
        for (lock, lf) in locks.iter() {
            let mut valid = true;
            for (kph, lph) in key.iter().zip(lock) {
                if kph + lph > 5 {
                    valid = false;
                    break;
                }
            }
            if !valid {
                continue;
            }
            valid_pairs += kf * lf;
        }
    }
    valid_pairs
}