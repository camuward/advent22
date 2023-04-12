use std::ops::RangeInclusive;

/// Parses the list of range pairs.
fn iter_pairs(
    input: &str,
) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input.lines().map(|line| {
        // split pair into two ranges
        let (left, right) = line.split_once(',').unwrap();

        // extract numbers from each range
        let [left, right] = [left, right]
            .map(|range| range.split_once('-').unwrap())
            .map(|(start, end)| [start, end].map(str::parse))
            .map(|parsed| parsed.map(Result::unwrap))
            .map(|[start, end]| start..=end);

        (left, right)
    })
}

pub fn part_one(input: &str) -> crate::Result<u32> {
    let redundant_pairs = iter_pairs(input).filter(|(left, right)| {
        let left_in_right = right.contains(left.start()) && right.contains(left.end());
        let right_in_left = left.contains(right.start()) && left.contains(right.end());

        left_in_right || right_in_left
    });

    Ok(redundant_pairs.count() as u32)
}

pub fn part_two(input: &str) -> crate::Result<u32> {
    let overlapping_pairs = iter_pairs(input).filter(|(left, right)| {
        left.contains(right.start())
            || left.contains(right.end())
            || right.contains(left.start())
            || right.contains(left.end())
    });

    Ok(overlapping_pairs.count() as u32)
}
