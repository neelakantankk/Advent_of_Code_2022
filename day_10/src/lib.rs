use std::error::Error;

mod part_01;
mod part_02;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let sum_of_signal_strengths = part_01::get_sum_of_signal_strengths(&contents);
    println!("The sum is {}", sum_of_signal_strengths);
    let display = part_02::get_display(&contents);
    part_02::draw_display(&display);
    Ok(())
}

