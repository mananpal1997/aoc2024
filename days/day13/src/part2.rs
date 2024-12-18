use crate::util::get_min_tokens;

pub fn solve(_input: &str) -> i64 {
    get_min_tokens(_input, i64::MAX, 10000000000000)
}