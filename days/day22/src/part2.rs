use std::collections::{HashMap, HashSet};

const MODULO: i64 = 0xFFFFFF;

pub fn solve(_input: &str) -> u64 {
    let mut max_of_sequences: HashMap<[i64; 4], i64> = HashMap::new();
    let mut seen: HashSet<(usize, [i64; 4])> = HashSet::new();

    _input.lines().enumerate().for_each(|(i, secret)| {
        let iterations = 2000;
        let mut new_secret = secret.parse::<i64>().unwrap();
        let mut sequence: [i64; 4] = [0; 4];
        for j in 0..iterations {
            let prev_secret = new_secret % 10;

            new_secret = ((new_secret << 6) ^ new_secret) & MODULO;
            new_secret = ((new_secret >> 5) ^ new_secret) & MODULO;
            new_secret = ((new_secret << 11) ^ new_secret) & MODULO;

            sequence.rotate_left(1);
            sequence[3] = new_secret % 10 - prev_secret;
            if j >= 3 {
                let key = (i, sequence);
                if seen.insert(key) {
                    *max_of_sequences.entry(sequence).or_default() += new_secret % 10;
                }
            }
        }
    });

    *max_of_sequences.values().max().unwrap() as u64
}