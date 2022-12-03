pub fn one(input: &str) -> u32 {
    input
        .as_bytes()
        .chunks(4)
        .map(|ln| (ln[0], ln[2]))
        .map(|(left, right)| {
            let choice_pts = right - b'W';
            let win_pts = match (left as char, right as char) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                _ => unreachable!(),
            };

            (win_pts + choice_pts) as u32
        })
        .sum()
}

pub fn two(input: &str) -> u32 {
    input
        .as_bytes()
        .chunks(4)
        .map(|ln| (ln[0] - b'A', ln[2] - b'X'))
        .map(|(left, right)| {
            let choice_pts = (2 + left + right) % 3 + 1;
            let win_pts = 3 * right;

            (win_pts + choice_pts) as u32
        })
        .sum()
}
