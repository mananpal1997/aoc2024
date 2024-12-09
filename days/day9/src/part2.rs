struct FileBlock {
    pos: usize,
    id: usize,
    count: usize
}

struct SpaceBlock {
    pos: usize,
    count: usize
}

pub fn solve(_input: &str) -> usize {
    let mut offset = 0;
    let mut file_blocks: Vec<FileBlock> = Vec::new();
    let mut space_blocks: Vec<SpaceBlock> = Vec::new();
    let mut expanded: Vec<usize> = Vec::new();
    _input.bytes().enumerate().map(|(idx, b)| (idx, (b - b'0') as usize)).for_each(|(idx, c)| {
        if idx % 2 == 0 {
            (0..c).for_each(|_| expanded.push(idx / 2));
            file_blocks.push(FileBlock{pos: offset, count: c, id: idx / 2});
        } else {
            (0..c).for_each(|_| expanded.push(0));
            space_blocks.push(SpaceBlock{pos: offset, count: c as usize});
        }
        offset += c;
    });

    for fb in file_blocks.iter_mut().rev() {
        for sb in space_blocks.iter_mut() {
            if sb.pos > fb.pos {
                break;
            }
            if sb.count >= fb.count {
                sb.count -= fb.count;
                (0..fb.count).for_each(|i| {
                    expanded[sb.pos + i] = fb.id;
                    expanded[fb.pos + i] = 0;
                });
                sb.pos += fb.count;
                break;
            }
        }
    }

    expanded.iter().enumerate().map(|(i, f_id)| i * f_id).sum()
}