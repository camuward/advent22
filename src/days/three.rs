fn unique_items(s: &str) -> u64 {
    s.bytes()
        .map(|b| match b {
            b'a'..=b'z' => 1 + b - b'a',
            b'A'..=b'Z' => 27 + b - b'A',
            _ => unreachable!(),
        })
        .fold(0, |acc, b| acc | (1u64 << b))
}

pub fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|bag| bag.split_at(bag.len() / 2))
        .map(|(l, r)| [l, r].map(unique_items))
        .map(|[l, r]| u64::trailing_zeros(l & r))
        .sum()
}

pub fn two(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|bags| bags.map(unique_items))
        .map(|[a, b, c]| a & b & c)
        .map(u64::trailing_zeros)
        .sum()
}
