#[derive(Debug)]
pub struct Step {
    pub count: usize,
    pub src: usize,
    pub dst: usize,
}

mod parse {
    use nom::bytes::complete::{take_while, take_while1};
    use nom::combinator::map_res;
    use nom::IResult;

    use super::*;

    /// Parses the drawing of stacks.
    pub fn initial_state(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
        Ok((
            input,
            vec![
                b"QFMRLWCV".to_vec(),
                b"DQL".to_vec(),
                b"PSRGWCNB".to_vec(),
                b"LCDHBQG".to_vec(),
                b"VGLFZS".to_vec(),
                b"DGNP".to_vec(),
                b"DZPVFCW".to_vec(),
                b"CPDMS".to_vec(),
                b"ZNWTVMPC".to_vec(),
            ],
        ))
    }

    /// Parses a single instruction to move a specified number of crates.
    pub fn movement_instruction(input: &str) -> IResult<&str, Step> {
        let (input, _) = take_while(char::is_whitespace)(input)?;

        let tag_num = |tag, input| {
            let (input, _) = nom::bytes::complete::tag(tag)(input)?;
            map_res(take_while1(char::is_numeric), str::parse)(input)
        };

        let (input, count) = tag_num("move ", input)?;
        let (input, src) = tag_num(" from ", input)?;
        let (input, dst) = tag_num(" to ", input)?;

        Ok((input, Step { count, src, dst }))
    }
}

pub fn part_one(input: &str) -> String {
    let (drawing, mut instructions) = input.split_at(input.find("move").unwrap());

    let (tx, rx) = std::sync::mpsc::channel();

    let stacks = std::thread::scope(|s| {
        s.spawn(move || {
            while !instructions.is_empty() {
                let (input, step) = parse::movement_instruction(instructions).unwrap();
                instructions = input;

                tx.send(step).unwrap();
            }
        });

        // parse the initial state
        let (_, init_stacks) = parse::initial_state(drawing).unwrap();

        std::iter::from_fn(|| rx.recv().ok())
            // process the buffered instructions (rx.recv())
            .fold(init_stacks, |mut stacks, Step { count, src, dst }| {
                let [src, dst] = stacks.get_many_mut([src - 1, dst - 1]).unwrap();

                // let src = &mut stacks[src - 1];
                // buf.extend(src.drain(src.len() - count..));

                // let dst = &mut stacks[dst - 1];
                // dst.extend(buf.drain(..).rev());
                // move the crates one by one to the destination stack
                let src_stack = src.drain(src.len() - count..);
                dst.extend(src_stack.rev()); // reverse the order

                stacks
            })
    });

    let top_crates = stacks.iter().filter_map(|crates| crates.last());
    top_crates.map(|&b| b as char).collect()
}

pub fn part_two(_input: &str) -> String {
    Default::default()
}
