pub fn part_one(input: &str) -> eyre::Result<usize> {
    let mut groups = input.as_bytes().windows(4);
    let marker_pos = groups.position(|group| {
        // check if the group is unique (thanks, LLVM!)
        for (idx, el) in group.iter().enumerate() {
            let mut after_el = group.iter().skip(idx + 1);
            if after_el.any(|other| el == other) {
                return false;
            }
        }

        true
    });

    Ok(marker_pos.expect("no start-of-packet marker") + 4)
}

pub fn part_two(input: &str) -> eyre::Result<usize> {
    todo!()
}
