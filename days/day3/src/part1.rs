use regex::Regex;

pub fn solve(_input: &str) -> i32 {
    let instruction_pattern = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();

    instruction_pattern
    .captures_iter(_input)
    .map(|m| m.extract())
    .map(|(_, [c])| {
        let ret: i32 = c.split(",").flat_map(str::parse::<i32>).product();
        ret
    })
    .sum()
}