const ROW_LENGTH: isize = 40;

pub fn get_display(contents: &str) -> Vec<Vec<&'static str>> {
    let mut display = vec![vec![" "; 40]; 6];
    let mut register_x = 1;
    let mut cycle_count = 0;
    for line in contents.lines() {
        match line.split_whitespace().next().unwrap() {
            "noop" => {
                draw_if_visible(&mut display, cycle_count, register_x);
                cycle_count += 1;
            }
            "addx" => {
                let v = line.split_whitespace().nth(1).unwrap();
                for _i in 0..2 {
                    draw_if_visible(&mut display, cycle_count, register_x);
                    cycle_count += 1;
                }
                register_x += v.parse::<isize>().unwrap();
            }
            _ => panic!("Unknown branch!"),
        }
    }
    display
}

fn draw_if_visible(display: &mut [Vec<&'static str>], cycle_count: isize, register_x: isize) {
    let current_pixel = cycle_count % ROW_LENGTH;
    if current_pixel >= (register_x - 1) && current_pixel <= (register_x + 1) {
        let row = (cycle_count / ROW_LENGTH) as usize;
        let column = (cycle_count % ROW_LENGTH) as usize;
        display[row][column] = "|";
    };
}

pub fn draw_display(display: &Vec<Vec<&'static str>>) {
    for row in display {
        let column = row.join("");
        println!("{}", column);
    }
}
