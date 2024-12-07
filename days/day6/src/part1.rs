use crate::util::{get_covered_cells, get_guard_pos};

pub fn solve(_input: &str) -> i32 {
    let lines: Vec<Vec<u8>> = _input.lines().map(|line| line.bytes().collect()).collect();
    let guard_pos = get_guard_pos(&lines);
    let (covered_cells, _) = get_covered_cells(&lines, guard_pos);
    covered_cells.len() as i32
}