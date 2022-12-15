use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let moves = parse_content(&contents);
    let visited_positions = get_count_of_visited_positions(&moves);
    println!(
        "Number of positions visited at least once: {}",
        visited_positions
    );
    Ok(())
}

fn parse_content(contents: &str) -> Vec<(isize, isize)> {
    let mut moves: Vec<(isize, isize)> = Vec::new();
    let unit_vecs = HashMap::from([("R", (1, 0)), ("L", (-1, 0)), ("U", (0, 1)), ("D", (0, -1))]);
    for line in contents.lines() {
        let direction = line.split_whitespace().next().unwrap();
        let steps = line.split_whitespace().last().unwrap();
        let steps: usize = steps.parse().unwrap();
        for _ in 0..steps {
            moves.push(*unit_vecs.get(&direction).unwrap());
        }
    }
    moves
}

fn get_count_of_visited_positions(moves: &[(isize, isize)]) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut position_tail: HashSet<(isize, isize)> = HashSet::new();
    position_tail.insert(tail);
    for instruction in moves.iter() {
        head = (head.0 + instruction.0, head.1 + instruction.1);
        tail = get_tail_position(head, tail);
        position_tail.insert(tail);
    }

    position_tail.len()
}

fn get_tail_position(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    if head.0 == tail.0 && (head.1).abs_diff(tail.1) == 2 {
        (tail.0, tail.1 + (head.1 - tail.1).signum())
    } else if head.1 == tail.1 && (head.0).abs_diff(tail.0) == 2 {
        (tail.0 + (head.0 - tail.0).signum(), tail.1)
    } else if head.0.abs_diff(tail.0) == 2 || head.1.abs_diff(tail.1) == 2 {
        (
            tail.0 + (head.0 - tail.0).signum(),
            tail.1 + (head.1 - tail.1).signum(),
        )
    } else {
        (tail.0, tail.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_tail_position() {
        let (head, tail) = ((0, 0), (0, 0));
        assert_eq!((0, 0), get_tail_position(head, tail));
        let (head, tail) = ((0, 2), (0, 0));
        assert_eq!((0, 1), get_tail_position(head, tail));
        let (head, tail) = ((1, 2), (3, 1));
        assert_eq!((2, 2), get_tail_position(head, tail));
    }
    fn make_data() -> &'static str {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }

    #[test]
    fn test_get_count_of_visited_positions() {
        let data = make_data();
        let moves = parse_content(data);
        let positions_visited = get_count_of_visited_positions(&moves);
        assert_eq!(13, positions_visited);
    }
}
