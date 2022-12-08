use crate::part_01::parse_instruction;
use std::collections::HashMap;

pub fn get_top_line(stacks: HashMap<u32, Vec<char>>, instructions: &str) -> String {
    let mut stacks = stacks;
    let mut topline = String::new();
    for instruction in instructions.lines() {
        let (number_of_crates, start, end) = parse_instruction(instruction);
        let crates = {
            let start_stack = stacks.entry(start).or_default();
            start_stack.split_off(start_stack.len() - (number_of_crates as usize))
        };
        let end_stack = stacks.entry(end).or_default();
        end_stack.extend(crates.iter());
    }
    let mut keys = stacks.keys().copied().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        topline.push(stacks.get_mut(&key).unwrap().pop().unwrap());
    }
    topline
}
