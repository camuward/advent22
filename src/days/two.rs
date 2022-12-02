fn split_line(input: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    let left = input.bytes().step_by(4);
    let right = input.bytes().skip(2).step_by(4);

    left.zip(right)
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

pub fn two(_input: &str) -> u32 {
    0
}
