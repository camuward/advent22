#![feature(slice_as_chunks, array_chunks, iter_array_chunks, get_many_mut)]

extern crate color_eyre as eyre;

use year2022::day05 as current;

#[allow(dead_code)]
mod year2022 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("input.txt")?;

    for func in [current::part_one, current::part_two] {
        let start = std::time::Instant::now();
        let res = func(&input);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }

    Ok(())
}
