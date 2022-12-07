use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let overlap_counter = get_count_of_overlaps(&contents);
    let intersection_counter = get_count_of_intersections(&contents);
    println!("Total overlaps: {}", overlap_counter);
    println!("Total intersections: {}", intersection_counter);
    Ok(())
}

fn has_overlap(line: &str) -> bool {
    let mut elves = Vec::new();
    for elf in line.split_terminator(',') {
        elves.push(get_assignment(elf));
    }
    elves[0].is_subset(&elves[1]) || elves[0].is_superset(&elves[1])
}

fn has_intersection(line: &str) -> bool {
    let mut elves = Vec::new();
    for elf in line.split_terminator(',') {
        elves.push(get_assignment(elf));
    }
    elves[0].intersection(&elves[1]).count() > 0
}

fn get_assignment(elf: &str) -> HashSet<u32> {
    let assignment: (u32, u32) = elf
        .split_terminator('-')
        .map(|i| i.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();
    (assignment.0..assignment.1 + 1).collect::<HashSet<u32>>()
}

fn get_count_of_overlaps(contents: &str) -> u32 {
    let mut counter = 0;
    for line in contents.lines() {
        if has_overlap(line) {
            counter += 1;
        };
    }
    counter
}

fn get_count_of_intersections(contents: &str) -> u32 {
    let mut counter = 0;
    for line in contents.lines() {
        if has_intersection(line) {
            counter += 1;
        };
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_assignment() {
        let elf = "2-3";
        let expected = (2..3 + 1).collect::<HashSet<u32>>();
        assert_eq!(expected, get_assignment(elf));
    }
    #[test]
    fn test_overlap() {
        let line1 = "2-8,3-7";
        let line2 = "6-6,4-6";
        let line3 = "2-4,6-8";
        assert!(has_overlap(line1));
        assert!(has_overlap(line2));
        assert!(!has_overlap(line3));
    }
    #[test]
    fn test_overlap_counter() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(2, get_count_of_overlaps(data));
    }
    #[test]
    fn test_intersection() {
        let line1 = "2-8,3-7";
        let line2 = "5-7,7-9";
        let line3 = "2-4,6-8";
        assert!(has_intersection(line1));
        assert!(has_intersection(line2));
        assert!(!has_intersection(line3));
    }
    #[test]
    fn test_intersection_counter() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(4, get_count_of_intersections(data));
    }
}
