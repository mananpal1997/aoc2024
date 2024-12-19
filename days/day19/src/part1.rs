use std::collections::{HashMap, HashSet};

pub fn solve(_input: &str) -> usize {
    let lines: Vec<_> = _input.lines().collect();

    let binding = lines[0].replace(" ", "");
    let configurations: HashSet<&str> = binding.split(",").collect();
    let mut memo: HashMap<String, bool> = HashMap::new();

    lines.iter().skip(2).filter(|towel| possible(&towel, &configurations, &mut memo)).count()
}

fn possible(remaining: &str, configurations: &HashSet<&str>, memo: &mut HashMap<String, bool>) -> bool {
    if remaining.is_empty() {
        return true;
    }
    if let Some(cached) = memo.get(remaining) {
        return *cached;
    }
    let mut ret = false;
    for &config in configurations {
        if remaining.starts_with(config) {
            let suffix = remaining.strip_prefix(config).unwrap_or("");
            ret |= possible(suffix, configurations, memo);
            if ret {
                break;
            }
        }
    }
    memo.insert(remaining.to_string(), ret);
    return ret;
}