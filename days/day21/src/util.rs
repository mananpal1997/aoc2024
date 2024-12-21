use std::collections::{HashMap, VecDeque};

pub fn helper(_input: &str, levels: usize) -> u64 {
    let numerical_pad_moves = HashMap::from([
        ('0', Vec::from([('2', '^'), ('A', '>')])),
        ('1', Vec::from([('2', '>'), ('4', '^')])),
        ('2', Vec::from([('1', '<'), ('0', 'v'), ('5', '^'), ('3', '>')])),
        ('3', Vec::from([('2', '<'), ('6', '^'), ('A', 'v')])),
        ('4', Vec::from([('7', '^'), ('5', '>'), ('1', 'v')])),
        ('5', Vec::from([('4', '<'), ('8', '^'), ('6', '>'), ('2', 'v')])),
        ('6', Vec::from([('9', '^'), ('5', '<'), ('3', 'v')])),
        ('7', Vec::from([('4', 'v'), ('8', '>')])),
        ('8', Vec::from([('7', '<'), ('5', 'v'), ('9', '>')])),
        ('9', Vec::from([('8', '<'), ('6', 'v')])),
        ('A', Vec::from([('3', '^'), ('0', '<')])),
    ]);

    let directional_pad_moves = HashMap::from([
        ('^', Vec::from([('A', '>'), ('v', 'v')])),
        ('A', Vec::from([('^', '<'), ('>', 'v')])),
        ('<', Vec::from([('v', '>')])),
        ('v', Vec::from([('<', '<'), ('^', '^'), ('>', '>')])),
        ('>', Vec::from([('A', '^'), ('v', '<')])),
    ]);

    let mut memo = HashMap::new();

    _input.lines().map(|line| {
        let code = line[..line.len()-1].parse::<u64>().unwrap();
        let sequence = line.chars().collect();

        code * min_moves_for_code(&sequence, levels, &numerical_pad_moves, &directional_pad_moves, true, &mut memo)
    }).sum()


}

fn min_moves_for_code(sequence: &Vec<char>, level: usize, numerical_pad_moves: &HashMap<char, Vec<(char, char)>>, directional_pad_moves: &HashMap<char, Vec<(char, char)>>, is_human: bool, memo: &mut HashMap<(Vec<char>, usize), u64>) -> u64 {
    let cache_key = (sequence.clone(), level);

    if let Some(&cached) = memo.get(&cache_key) {
        return cached;
    }

    let mut num_moves = 0;

    let pad_moves = match is_human {
        true => numerical_pad_moves,
        false => directional_pad_moves
    };

    for i in 0..sequence.len() {
        let (src, dst) = match i == 0 {
            true => ('A', sequence[i]),
            false => (sequence[i-1], sequence[i])
        };
        
        if level == 0 {
            num_moves += get_shortest_paths(&src, &dst, pad_moves).iter().map(|p| p.len() as u64).min().unwrap();
        } else {
            let mut temp = u64::MAX;
            for path in get_shortest_paths(&src, &dst, pad_moves) {
                temp = temp.min(min_moves_for_code(&path, level - 1, numerical_pad_moves, directional_pad_moves, false, memo));
            }
            num_moves += temp;
        }
    }

    memo.insert(cache_key, num_moves);
    num_moves
}

fn get_shortest_paths(src: &char, dst: &char, pad_moves: &HashMap<char, Vec<(char, char)>>) -> Vec<Vec<char>> {
    let mut queue = VecDeque::from([(src, Vec::new())]);

    let mut paths = Vec::new();
    let mut shortest_path_len = usize::MAX;

    while let Some((cur, path)) = queue.pop_front() {
        if cur == dst {
            if path.len() <= shortest_path_len {
                shortest_path_len = path.len();
                let mut sequence = path.clone();
                sequence.push('A');
                paths.push(sequence);
            }
        }

        if shortest_path_len != usize::MAX && path.len() > shortest_path_len {
            continue;
        }

        if let Some(moves) = pad_moves.get(cur) {
            for (adj, mov) in moves {
                let mut next_path = path.clone();
                next_path.push(*mov);
                queue.push_back((adj, next_path));
            }
        }
    }

    paths
}