use std::error::Error;
use std::collections::HashSet;

pub fn run(contents: String) -> Result<(),Box<dyn Error>> {

    let sum_of_signal_strengths = get_sum_of_signal_strengths(&contents);
    println!("The sum is {}",sum_of_signal_strengths);
    Ok(())
}

fn get_sum_of_signal_strengths(contents: &str) -> isize {
    let mut register_x = 1;
    let mut cycle_count = 0;
    let mut signal_strength = 0;

    for line in contents.lines() {
        match line.split_whitespace().nth(0).unwrap() {
            "noop" => {
                cycle_count +=1;
                signal_strength = check_signal_strength(signal_strength, cycle_count, register_x);
            },
            "addx" => {
                let v = line.split_whitespace().nth(1).unwrap();
                for _i in 0..2 {
                    cycle_count+=1;
                    signal_strength = check_signal_strength(signal_strength, cycle_count, register_x);
                };
                register_x += v.parse::<isize>().unwrap();
            },
            _ => {panic!("Should not have found this branch!");}
        }
    }
    signal_strength
}

fn check_signal_strength(current_signal_strength: isize, cycle_count: isize, register_x: isize) -> isize {
    let cycles_needed = HashSet::from([20,60,100,140,180,220]);
    if cycles_needed.contains(&cycle_count) {
        current_signal_strength + (cycle_count*register_x)
    } else {
        current_signal_strength
    }
}
