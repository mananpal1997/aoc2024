use std::collections::HashSet;

pub fn can_traverse(levels: &[i32], max_deletions_allowed: i32) -> bool {
    let allowed_increasing: HashSet<i32> = HashSet::from([1, 2, 3]);
    let allowed_decreasing: HashSet<i32> = HashSet::from([-1, -2, -3]);

    for i in 0..(max_deletions_allowed+1) {
        for vec in delete_n(levels, i) {
            if vec.windows(2).all(|win| allowed_decreasing.contains(&(win[0] - win[1]))) {
                return true;
            }
            if vec.windows(2).all(|win| allowed_increasing.contains(&(win[0] - win[1]))) {
                return true;
            }
        }
    }
    false
}

fn delete_n(vector: &[i32], n: i32) -> Vec<Vec<i32>> {
    if n == 0 {
        return vec![vector.to_owned()];
    }

    let mut result = Vec::new();
    for i in 0..vector.len() {
        let mut new_vec = vector.to_owned();
        new_vec.remove(i);
        result.extend(delete_n(&new_vec, n - 1));
    }
    result
}