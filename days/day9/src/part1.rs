pub fn solve(_input: &str) -> usize {
    let mut expanded: Vec<usize> = Vec::new();
    let mut l = 0;
    _input.bytes().enumerate().map(|(idx, b)| (idx, (b - b'0') as usize)).for_each(|(idx, c)| {
        if idx % 2 == 0 {
            if l == 0 {
                l = c;
            }
            (0..c).for_each(|_| expanded.push(idx / 2));
        } else {
            (0..c).for_each(|_| expanded.push(0));
        }
    });

    let mut r = expanded.len() - 1;
    while l < r {
        while l < r && expanded[l] != 0 {
            l += 1;
        }
        while r > l && expanded[r] == 0 {
            r -= 1;
        }
        if l >= r {
            break;
        }
        expanded[l] = expanded[r];
        expanded[r] = 0;
    }
    
    expanded.iter().enumerate().map(|(i, f_id)| i * f_id).sum()
}