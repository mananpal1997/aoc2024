use crate::util::can_traverse;

pub fn solve(_input: &str) -> i32 {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in _input.lines() {
        reports.push(Vec::from_iter(line.split_whitespace().map(|n| n.parse().unwrap())));
    }

    reports
    .iter()
    .filter(|&report| can_traverse(report, 0))
    .count() as i32
}