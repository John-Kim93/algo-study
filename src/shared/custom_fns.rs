/* ================= 
    IO Functions
================= */
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

/* ================= 
    Type Parsing Functions
================= */
fn parse_u32(string_value: &str) -> u32 {
    string_value.trim().parse::<u32>().expect("Failed to parse input as u32")
}

fn parse_f64(string_value: &str) -> f64 {
    string_value.trim().parse::<f64>().expect("Failed to parse input as f64")
}

/* ================= 
    String Control Functions
================= */
fn split_input(string_value: &str) -> Vec<String> {
    string_value
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
