use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let mut data_iter = contents.split_terminator("\n\n");
    let stacks = get_stacks(data_iter.next().unwrap());
    let instructions = data_iter.next().unwrap();

    let top_line = get_top_line(stacks, instructions);
    println!("Top line of crates: {}", top_line);

    Ok(())
}

fn get_top_line(stacks: HashMap<u32, Vec<char>>, instructions: &str) -> String {
    let mut stacks = stacks;
    let mut topline = String::new();
    for instruction in instructions.lines() {
        let (number_of_crates, start, end) = parse_instruction(instruction);
        for _i in 0..number_of_crates {
            let crate_name = {
                let start_stack = stacks.entry(start).or_default();
                start_stack.pop().unwrap()
            };
            let end_stack = stacks.entry(end).or_default();
            end_stack.push(crate_name);
        }
    }
    let mut keys = stacks.keys().copied().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        topline.push(stacks.get_mut(&key).unwrap().pop().unwrap());
    }
    topline
}

fn parse_instruction(instruction: &str) -> (u32, u32, u32) {
    instruction
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|c| c.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn get_stacks(data: &str) -> HashMap<u32, Vec<char>> {
    let mut stacks = HashMap::new();
    let mut lines = data.lines().rev();
    let keys = lines.next().unwrap().split_whitespace().collect::<Vec<_>>();
    for key in keys {
        stacks.insert(key.parse::<u32>().unwrap(), Vec::new());
    }
    for line in lines {
        parse_stack_line(&mut stacks, line);
    }
    stacks
}

fn parse_stack_line(stacks: &mut HashMap<u32, Vec<char>>, line: &str) {
    for (index, letter) in line.char_indices().step_by(4) {
        if !letter.is_whitespace() {
            let stack_num: u32 = ((index as u32) / 4) + 1;
            let crate_letter = char::from_str(&line[index + 1..index + 2]).unwrap();
            stacks
                .entry(stack_num)
                .and_modify(|crates| crates.push(crate_letter));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_empty_stack() -> HashMap<u32, Vec<char>> {
        let mut stacks = HashMap::new();
        for i in 1..10 {
            stacks.insert(i, Vec::new());
        }
        stacks
    }

    fn create_final_result(line: &str) -> HashMap<u32, Vec<char>> {
        let mut stacks = HashMap::new();
        for (i, c) in line.char_indices() {
            stacks.insert((i + 1) as u32, vec![c]);
        }
        stacks
    }

    #[test]
    fn test_parse_stack_line() {
        let mut empty_stacks = create_empty_stack();
        let expected = create_final_result("DHRLNWGCR");
        let test_line = "[D] [H] [R] [L] [N] [W] [G] [C] [R]";
        parse_stack_line(&mut empty_stacks, test_line);
        assert_eq!(expected, empty_stacks);
    }

    #[test]
    fn test_get_top_line() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let mut data_iter = data.split_terminator("\n\n");
        let stacks = get_stacks(data_iter.next().unwrap());
        let instructions = data_iter.next().unwrap();
        let topline = get_top_line(stacks, instructions);

        assert_eq!("CMZ", topline);
    }
}
