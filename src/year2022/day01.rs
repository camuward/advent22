fn sum_cals<'a>(elf: &str) -> eyre::Result<u32> {
    let mut cals = elf.lines().map(|cals| cals.parse::<u32>());
    cals.try_fold(0, |acc, cal| Ok(acc + cal?)) // sum
}

pub fn part_one(input: &str) -> eyre::Result<u32> {
    let elves = input.split("\n\n");
    let mut cals = elves.map(sum_cals);

    cals.try_fold(u32::MIN, |max, cals| Ok(max.max(cals?)))
}

pub fn part_two(input: &str) -> eyre::Result<u32> {
    let elves = input.split("\n\n");
    let mut cals = elves.map(sum_cals);
    let mut max_3 = [u32::MIN; 3]; // lowest -> highest
    
    while let Some(next) = cals.next().transpose()? {
        if let Some(pos) = max_3.iter().rposition(|&max| next > max) {
            max_3[..=pos].rotate_left(1);
            max_3[pos] = next;
        }
    }

    Ok(max_3.into_iter().sum())
}
