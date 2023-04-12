pub fn part_one(input: &str) -> crate::Result<u32> {
    Ok(input
        .as_bytes()
        .array_chunks()
        .map(|&[left, _, right, _]| {
            let choice_pts = right - b'W';
            let win_pts = match (left as char, right as char) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                _ => unreachable!(),
            };

            (win_pts + choice_pts) as u32
        })
        .sum())
}

pub fn part_two(input: &str) -> crate::Result<u32> {
    Ok(input
        .as_bytes()
        .array_chunks()
        .map(|&[l, _, r, _]| (l - b'A', r - b'X'))
        .map(|(left, right)| {
            let choice_pts = (2 + left + right) % 3 + 1;
            let win_pts = 3 * right;

            (win_pts + choice_pts) as u32
        })
        .sum())
}
