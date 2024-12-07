use std::{cmp::Ordering, collections::HashSet};

pub fn solve(_input: &str) -> i32 {
    let page_ordering: HashSet<(&str, &str)> = _input
    .lines()
    .filter(|line| line.contains("|"))
    .map(|line| line.split_once("|").unwrap())
    .collect();
    
    _input
    .lines()
    .filter(|line| line.contains(","))
    .map(|line| Vec::from_iter(line.split(",")))
    .filter(|pages| {
        let mut pages_cpy = pages.clone();
        pages_cpy
        .sort_by(|&a, &b| {
            match page_ordering.contains(&(a, b)) {
                true => Ordering::Less,
                false => Ordering::Greater,
            }
        });
        pages_cpy == *pages
    })
    .map(|pages| pages[pages.len() / 2].parse::<i32>().unwrap())
    .sum()
}