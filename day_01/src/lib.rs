use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let max_cal = part_one(&contents);
    println!("The maximum calories are {}", max_cal);

    let total_cal = part_two(&contents);
    println!("The total calories are {}", total_cal);
    Ok(())
}

fn part_one(contents: &str) -> u32 {
    let elf_calories = get_elf_calories(contents);
    *elf_calories.iter().max().unwrap()
}

fn part_two(contents: &str) -> u32 {
    let mut elf_calories = get_elf_calories(contents);
    elf_calories.sort();
    elf_calories.reverse();
    elf_calories[..3].iter().sum()
}

fn get_elf_calories(contents: &str) -> Vec<u32> {
    let elves: Vec<_> = contents.split_terminator("\n\n").collect();
    let mut elf_calories = Vec::new();

    for elf in elves {
        let total_calories: u32 = elf.lines().map(|cal| cal.parse::<u32>().unwrap()).sum();
        elf_calories.push(total_calories);
    }
    elf_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        let test_data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        test_data
    }

    #[test]
    fn test_p1() {
        let test_data = get_test_data();
        assert_eq!(24000, part_one(test_data));
    }
    #[test]
    fn test_p2() {
        let test_data = get_test_data();
        assert_eq!(45000, part_two(test_data));
    }
}
