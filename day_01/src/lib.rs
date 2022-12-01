use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let max_cal = part_one(&contents);
    println!("The maximum calories are {}", max_cal);
    Ok(())
}

fn part_one(contents: &str) -> u32 {
    let elves: Vec<_> = contents.split_terminator("\n\n").collect();
    let mut elf_calories = Vec::new();

    for elf in elves {
        let total_calories: u32 = elf.lines().map(|cal| cal.parse::<u32>().unwrap()).sum();
        elf_calories.push(total_calories);
    }

    *elf_calories.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
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
        assert_eq!(24000, part_one(test_data));
    }
}
