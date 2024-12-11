use std::collections::HashMap;

pub fn mutate(_input: &str, iterations: i32) -> i64 {
    let mut stones: HashMap<i64, i64> = HashMap::new();
    _input.split_whitespace().flat_map(str::parse::<i64>).for_each(|stone| *stones.entry(stone).or_insert(0) += 1);

    let equal_split = |num: i64| -> Option<(i64, i64)> {
        let num_diigts = num.ilog10() + 1;
        if num_diigts % 2 == 1 {
            return None
        }
        let base: i64 = 10;
        return Some((num / (base.pow(num_diigts / 2)), num % (base.pow(num_diigts / 2))))
    };

    (0..iterations).for_each(|_| {
        let temp: Vec<(i64, i64)> = stones.drain().collect();
        for (stone, cnt) in temp {
            if stone == 0 {
                *stones.entry(1).or_insert(0) += cnt;
            } else {
                match equal_split(stone) {
                    Some((left, right)) => {
                        *stones.entry(left).or_default() += cnt;
                        *stones.entry(right).or_default() += cnt;
                    },
                    None => {
                        *stones.entry(stone * 2024).or_default() += cnt;
                    }
                }
            }
        }
    });

    stones.values().sum()
}