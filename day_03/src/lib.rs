use std::collections::HashSet;
use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let priority_sum = get_priority_sum(&contents);
    println!("The total priority is {}", priority_sum);
    Ok(())
}

fn get_priority_sum(contents: &str) -> u32 {
    let mut sum_of_priorities = 0;
    for line in contents.lines() {
        let line_priority = get_line_priority(line);
        sum_of_priorities += line_priority;
    }
    sum_of_priorities
}

fn get_line_priority(line: &str) -> u32 {
    let first_compartment: HashSet<char> = line[..line.len() / 2].chars().collect();
    let second_compartment: HashSet<char> = line[line.len() / 2..].chars().collect();
    let mistake = first_compartment
        .intersection(&second_compartment)
        .next()
        .unwrap();
    if mistake.is_lowercase() {
        (*mistake as u32) - ('a' as u32) + 1
    } else {
        (*mistake as u32) - ('A' as u32) + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_priority() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(16, get_line_priority(&line));
    }
    #[test]
    fn test_total_priority() {
        let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, get_priority_sum(&lines));
    }
}
