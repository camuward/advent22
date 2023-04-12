#![feature(test, slice_as_chunks, array_chunks, iter_array_chunks, get_many_mut)]

#[macro_use]
extern crate eyre;
extern crate test;

use std::{fs, time};

use year2022::day06 as current;

#[allow(dead_code)]
mod year2022 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
}

type Result<T> = color_eyre::Result<T>;

fn main() -> crate::Result<()> {
    color_eyre::install()?;

    let input = fs::read_to_string("input.txt")?;

    for func in [current::part_one, current::part_two] {
        let start = time::Instant::now();
        let res = func(&input)?;
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }

    Ok(())
}

#[bench]
fn part_one(b: &mut test::Bencher) {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    b.iter(|| current::part_one(&input).unwrap())
}

#[bench]
fn part_two(b: &mut test::Bencher) {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    b.iter(|| current::part_two(&input).unwrap())
}
