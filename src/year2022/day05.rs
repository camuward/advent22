#[derive(Debug)]
pub struct Step {
    pub count: usize,
    pub src: usize,
    pub dst: usize,
}

mod parse {
    use super::*;

    /// Parses the drawing of stacks.
    pub fn initial_state(drawing: &str) -> crate::Result<Vec<Vec<u8>>> {
        let (content, labels) = drawing.rsplit_once(']').expect("no crates in drawing");
        let count_cols = labels
            .split_whitespace()
            .map_while(|num| num.parse().ok())
            .last()
            .expect("no labels in drawing");

        let mut stacks: Vec<Vec<u8>> = vec![vec![]; count_cols];
        for row in content.lines().rev() {
            for (row_index, cell) in row.bytes().skip(1).step_by(4).enumerate() {
                if !cell.is_ascii_whitespace() {
                    stacks[row_index].push(cell);
                }
            }
        }

        Ok(stacks)
    }

    /// Iterate over the provided list of instructions.
    pub fn iter_steps(list: &str) -> impl Iterator<Item = Step> + '_ {
        list.split_whitespace()
            .filter_map(|num| num.parse().ok())
            .array_chunks()
            .map(|[count, src, dst]| Step { count, src, dst })
    }
}

pub fn part_one(input: &str) -> crate::Result<String> {
    let (drawing, instructions) = input.split_at(input.find("move").unwrap());
    let mut stacks = parse::initial_state(drawing).unwrap();

    for Step { count, src, dst } in parse::iter_steps(instructions) {
        let [src, dst] = stacks.get_many_mut([src - 1, dst - 1]).unwrap();

        // move the crates one by one to the destination stack
        let src_stack = src.drain(src.len() - count..);
        dst.extend(src_stack.rev()); // reverse the order
    }

    // get the top-most crate for each stack
    let top_crates = stacks.iter().filter_map(|crates| crates.last());
    Ok(top_crates.map(|&b| b as char).collect())
}

pub fn part_two(input: &str) -> crate::Result<String> {
    let (drawing, instructions) = input.split_at(input.find("move").unwrap());
    let mut stacks = parse::initial_state(drawing).unwrap();

    for Step { count, src, dst } in parse::iter_steps(instructions) {
        let [src, dst] = stacks.get_many_mut([src - 1, dst - 1]).unwrap();

        // move the crates one by one to the destination stack
        dst.extend(src.drain(src.len() - count..));
    }

    // get the top-most crate for each stack
    let top_crates = stacks.iter().filter_map(|crates| crates.last());
    Ok(top_crates.map(|&b| b as char).collect())
}
