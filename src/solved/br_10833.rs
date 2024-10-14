// mod shared;

use std::io;
// use shared::custom_fns::{ parse_u32, parse_f64, split_input, read_input };

// FROM MY FUNCTION MODULES
/* ================= 
    IO Functions
================= */
pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

/* ================= 
    Type Parsing Functions
================= */
pub fn parse_u32(string_value: &str) -> u32 {
    string_value.trim().parse::<u32>().expect("Failed to parse input as u32")
}

pub fn parse_f64(string_value: &str) -> f64 {
    string_value.trim().parse::<f64>().expect("Failed to parse input as f64")
}

/* ================= 
    String Control Functions
================= */
pub fn split_input(string_value: &str) -> Vec<String> {
    string_value
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let tc = parse_u32(&read_input());

    let mut rest_apple = 0;

    for _ in 0..tc {
        if let [students, apples] = &split_input(&read_input())[..] {
            let number_of_students = parse_u32(&students);
            let number_of_apples = parse_u32(&apples);

            rest_apple += number_of_apples % number_of_students;
        } else {
            println!("Input does not have exactly 2 values.");
        }
    }

    println!("{}", rest_apple);
}
