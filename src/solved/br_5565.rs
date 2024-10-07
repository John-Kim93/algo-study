use std::io;

fn read_num_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse::<u32>()
        .expect("Failed to parse input as u32")
}

fn main() {
    let mut val = read_num_input();

    for _ in 0..9 {
        let cur_val = read_num_input();
        val -= cur_val;
    }
    println!("{}", val);
}
