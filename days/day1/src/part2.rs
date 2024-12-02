use std::collections::HashMap;

pub fn solve(_input :&str) -> i32 {
    let mut mp1: HashMap<i32, i32> = HashMap::new();
    let mut mp2: HashMap<i32, i32> = HashMap::new();

    for line in _input.lines() {
        let mut numbers = line.split_whitespace();
        if let(Some(num1), Some(num2)) = (numbers.next(), numbers.next()) {
            if let(Ok(n1), Ok(n2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                *mp1.entry(n1).or_insert(0) += 1;
                *mp2.entry(n2).or_insert(0) += 1;
            }
        }
    }

    mp1
    .iter()
    .filter_map(|(n1, f1)| mp2.get(n1).map(|&f2| n1 * f1 * f2))
    .sum()
}