fn split_line(input: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    let left = input.bytes();
    let right = input.bytes().skip(2);

    left.zip(right).step_by(4)
}

pub fn one(input: &str) -> u32 {
    split_line(input)
        .map(|(left, right)| {
            let score = match (left as char, right as char) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                _ => unreachable!(),
            };

            score + (right - b'W') as u32
        })
        .sum()
}

pub fn two(input: &str) -> u32 {
    split_line(input)
        .map(|(l, r)| (l - b'A', r - b'X'))
        .map(|(left, right)| {
            let choice = (2 + left + right) % 3 + 1;
            (3 * right + choice) as u32
        })
        .sum()
}
