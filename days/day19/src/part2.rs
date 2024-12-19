use std::collections::{HashMap, HashSet};

pub fn solve(_input: &str) -> usize {
    let lines: Vec<_> = _input.lines().collect();

    let binding = lines[0].replace(" ", "");
    let configurations: HashSet<&str> = binding.split(",").collect();
    let mut memo: HashMap<String, usize> = HashMap::new();

    lines.iter().skip(2).map(|towel| possible(&towel, &configurations, &mut memo)).sum()
}

fn possible(remaining: &str, configurations: &HashSet<&str>, memo: &mut HashMap<String, usize>) -> usize {
    if remaining.is_empty() {
        return 1;
    }
    if let Some(cached) = memo.get(remaining) {
        return *cached;
    }
    let mut ret = 0;
    for &config in configurations {
        if remaining.starts_with(config) {
            let suffix = remaining.strip_prefix(config).unwrap_or("");
            ret = ret + possible(suffix, configurations, memo);
        }
    }
    memo.insert(remaining.to_string(), ret);
    return ret;
}