fn sum_cals<'a>(elf: impl Iterator<Item = &'a str>) -> u32 {
    elf.filter_map(|cals| cals.parse::<u32>().ok()).sum()
}

pub fn part_one(input: &str) -> u32 {
    let elves = input.split("\n\n");
    let cals = elves.map(str::lines).map(sum_cals);
    
    cals.max().unwrap_or(0)
}

pub fn part_two(input: &str) -> u32 {
    let elves = input.split("\n\n");
    let cals = elves.map(str::lines).map(sum_cals);

    let (mut a, mut b, mut c) = (0, 0, 0);
    for d in cals {
        use std::mem::replace as rep;
        c = match d {
            _ if d > a => rep(&mut b, rep(&mut a, d)),
            _ if d > b => rep(&mut b, d),
            _ if d > c => d,
            _ => c,
        };
    }

    a + b + c
}
