#![feature(iter_array_chunks)]

mod days;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    use days::three as current;
    for func in [current::one, current::two] {
        let start = std::time::Instant::now();
        let res = func(&input);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }

    Ok(())
}
