use regex::Regex;

pub fn solve(_input: &str) -> i32 {
    let instructions = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3},\d{1,3})\)").unwrap();
    
    let mut enabled = true;

    instructions
    .captures_iter(_input)
    .map(|m| m.extract())
    .map(|(_, [c])| {
        let mut ret = 0;
        match c {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            mul_instr => {
                if enabled {
                    ret = mul_instr.split(",").flat_map(str::parse::<i32>).product();
                }
            }
        }
        ret
    }).sum()
}
