use days::one as current;

mod days;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    println!("{}", current::one(&input));
    println!("{}", current::two(&input));

    Ok(())
}
