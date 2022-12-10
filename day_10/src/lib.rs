use std::error::Error;

pub fn run(contents: String) -> Result<(),Box<dyn Error>> {

    let sum_of_signal_strengths =part_01::get_sum_of_signal_strengths(&contents);
    println!("The sum is {}",sum_of_signal_strengths);
    let display = part_02::get_display(&contents);
    part_02::draw_display(&display);
    Ok(())
}

mod part_01;
mod part_02;
