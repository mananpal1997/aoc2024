use crate::util::{get_covered_cells, get_guard_pos};

pub fn solve(_input: &str) -> i32 {
    let mut lines: Vec<Vec<u8>> = _input.lines().map(|line| line.bytes().collect()).collect();
    let guard_pos = get_guard_pos(&lines);
    
    get_covered_cells(&lines, guard_pos).0
    .iter()
    .filter(|&cell| {
        lines[cell.0 as usize][cell.1 as usize] = b'#';
        let (_, is_loop) = get_covered_cells(&lines, guard_pos);
        lines[cell.0 as usize][cell.1 as usize] = b'.';
        is_loop
    })
    .count() as i32
}