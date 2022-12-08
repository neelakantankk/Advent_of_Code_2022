use std::error::Error;
mod part_01;
mod part_02;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let mut data_iter = contents.split_terminator("\n\n");
    let stacks = part_01::get_stacks(data_iter.next().unwrap());
    let instructions = data_iter.next().unwrap();

    let top_line = part_01::get_top_line(stacks, instructions);
    println!("Part 01: Top line of crates: {}", top_line);

    let mut data_iter = contents.split_terminator("\n\n");
    let stacks = part_01::get_stacks(data_iter.next().unwrap());
    let instructions = data_iter.next().unwrap();

    let top_line = part_02::get_top_line(stacks, instructions);
    println!("Part 02: Top line of crates: {}", top_line);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part_01;
    use crate::part_02;
    use std::collections::HashMap;

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
        part_01::parse_stack_line(&mut empty_stacks, test_line);
        assert_eq!(expected, empty_stacks);
    }

    fn create_data() -> &'static str {
        "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    #[test]
    fn test_get_top_line_p1() {
        let data = create_data();
        let mut data_iter = data.split_terminator("\n\n");
        let stacks = part_01::get_stacks(data_iter.next().unwrap());
        let instructions = data_iter.next().unwrap();
        let topline = part_01::get_top_line(stacks, instructions);

        assert_eq!("CMZ", topline);
    }

    #[test]
    fn test_get_top_line_p2() {
        let data = create_data();
        let mut data_iter = data.split_terminator("\n\n");
        let stacks = part_01::get_stacks(data_iter.next().unwrap());
        let instructions = data_iter.next().unwrap();
        let topline = part_02::get_top_line(stacks, instructions);

        assert_eq!("MCD", topline);
    }
}
