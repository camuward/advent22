pub fn part_one(input: &str) -> eyre::Result<usize> {
    let mut groups = input.as_bytes().windows(4);
    let index = groups.position(|group| {
        !group
            .iter()
            .enumerate()
            .any(|(idx, left)| group.iter().skip(idx + 1).any(|right| left == right))
    });

    Ok(index.expect("no start-of-message marker") + 4)
}

pub fn part_two(input: &str) -> eyre::Result<usize> {
    todo!()
}
