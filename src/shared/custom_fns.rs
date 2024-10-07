fn read_num_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse::<u32>()
        .expect("Failed to parse input as u32")
}