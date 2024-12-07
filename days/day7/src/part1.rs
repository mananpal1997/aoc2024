pub fn solve(_input: &str) -> i64 {
    let possible = |nums: &Vec<i64>, target: i64| -> bool {
        let idx = nums.len() - 1;
        let mut states: Vec<(i64, usize)> = Vec::from([(target, idx)]);

        while !states.is_empty() {
            let (val, curr_idx) = states.pop().unwrap();
            if curr_idx == 0 {
                if val == nums[0] {
                    return true;
                }
                continue;
            }
            if val % nums[curr_idx] == 0 {
                states.push((val / nums[curr_idx], curr_idx - 1));
            }
            if val >= nums[curr_idx] {
                states.push((val - nums[curr_idx], curr_idx - 1));
            }
        }
        return false;
    };

    _input
    .lines()
    .map(|line| line
        .split_once(":")
        .map(|(target, nums)| (
            target.parse::<i64>().unwrap(),
            nums.trim().split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect()
        ))
        .unwrap() as (i64, Vec<i64>)
    )
    .filter(|(target, nums)| possible(nums, *target))
    .map(|(target, _)| target)
    .sum()
}