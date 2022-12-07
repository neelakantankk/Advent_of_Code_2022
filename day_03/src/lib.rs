use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let priority_sum = part_01::get_priority_sum(&contents);
    let group_priority_sum = part_02::get_priority_sum(&contents);
    println!("The total priority is {}", priority_sum);
    println!("The total priority for groups is {}", group_priority_sum);
    Ok(())
}

mod part_01 {
    use super::*;
    use std::collections::HashSet;

    pub fn get_priority_sum(contents: &str) -> u32 {
        let mut sum_of_priorities = 0;
        for line in contents.lines() {
            let line_priority = get_line_priority(line);
            sum_of_priorities += line_priority;
        }
        sum_of_priorities
    }

    pub fn get_line_priority(line: &str) -> u32 {
        let first_compartment: HashSet<char> = line[..line.len() / 2].chars().collect();
        let second_compartment: HashSet<char> = line[line.len() / 2..].chars().collect();
        let mistake = first_compartment
            .intersection(&second_compartment)
            .next()
            .unwrap();
        parse_priority(mistake)
    }
}

fn parse_priority(c: &char) -> u32 {
    if c.is_lowercase() {
        (*c as u32) - ('a' as u32) + 1
    } else {
        (*c as u32) - ('A' as u32) + 27
    }
}

mod part_02 {
    use super::*;
    use std::collections::HashSet;

    pub fn get_priority_sum(contents: &str) -> u32 {
        let mut sum_of_priorities = 0;
        let groups: Vec<_> = contents
            .lines()
            .map(|line| line.chars().collect::<HashSet<_>>())
            .collect();
        for index in (0..groups.len()).step_by(3) {
            let group = &groups[index..index + 3];
            let group_sum = get_group_sum(group);
            sum_of_priorities += group_sum;
        }
        sum_of_priorities
    }

    pub fn get_group_sum(group: &[HashSet<char>]) -> u32 {
        let first_two_intersection: HashSet<char> =
            group[0].intersection(&group[1]).copied().collect();
        let mistake = first_two_intersection
            .intersection(&group[2])
            .next()
            .unwrap();
        parse_priority(mistake)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_priority() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(16, part_01::get_line_priority(&line));
    }
    #[test]
    fn test_total_priority_p1() {
        let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, part_01::get_priority_sum(&lines));
    }
    #[test]
    fn test_group_priority() {
        let group = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        assert_eq!(18, part_02::get_priority_sum(group));
    }
}
