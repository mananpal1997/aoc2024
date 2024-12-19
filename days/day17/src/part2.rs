use std::collections::VecDeque;

pub fn solve(_input: &str) -> u64 {
    let lines: Vec<_> = _input.lines().collect();

    let mut a: u64 = lines[0].split_once(": ").unwrap().1.parse::<u64>().unwrap();
    let mut b: u64 = lines[1].split_once(": ").unwrap().1.parse::<u64>().unwrap();
    let mut c: u64 = lines[2].split_once(": ").unwrap().1.parse::<u64>().unwrap();

    let program: Vec<u8> = lines[4].split_once(": ").unwrap().1.split(",").map(|c| c.as_bytes()[0] - b'0').collect();

    let combo = |op: u64, a: u64, b: u64, c: u64| -> u64 {
        if op == 4 {
            a
        } else if op == 5 {
            b
        } else if op == 6 {
            c
        } else {
            op
        }
    };

    let a_orig = a.clone();
    let b_orig = b.clone();
    let c_orig = c.clone();

    let n = program.len() as isize;

    let mut transitions = VecDeque::from([(0, 0)]);
    while !transitions.is_empty() {
        let (new_a, depth) = transitions.pop_front().unwrap();
        if new_a == a_orig {
            continue;
        }
        a = new_a;
        b = b_orig;
        c = c_orig;

        let mut ip: isize = 0;
        let mut new_out = Vec::new();
        while ip < n {
            let instruction = program[ip as usize];
            let operand = program[(ip + 1) as usize] as u64;
    
            match instruction {
                0 => a >>= combo(operand, a, b, c),
                1 => b ^= operand,
                2 => b = combo(operand, a, b, c) & 7,
                3 => {
                    if a != 0 {
                        ip = (operand as isize) - 2;
                    }
                },
                4 => b ^= c,
                5 => new_out.push((combo(operand, a, b, c) & 7) as u8),
                6 => b = a >> combo(operand, a, b, c),
                7 => c = a >> combo(operand, a, b, c),
                _ => {}
            }
    
            ip += 2;
        }
        if new_out == program {
            return new_a;
        }
        if depth == 0 || program.ends_with(&new_out) {
            for n in 0..8 {
                transitions.push_back((8 * new_a + n, depth + 1));
            }
        }
    }

    0
}