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
    let mut points: [u32; 8] = [0; 8];
    for i in 0..8 {
        points[i] = parse_u32(&read_input());
    }

    let mut answer_points = 0;
    let mut big_num_flags = [0; 8];
    for (index_01, value_01) in points.iter().enumerate() {
        let mut cnt = 0;
        for (index_02, value_02) in points.iter().enumerate() {
            if index_01 == index_02 {
                continue;
            }

            if cnt == 3 {
                break;
            }

            if *value_01 > *value_02 {
                cnt += 1;
            }
        }
        if cnt == 3 {
            big_num_flags[index_01] = 1;
            answer_points += *value_01;
        }
    }

    println!("{}", answer_points);
    let mut index_list: Vec<String> = Vec::new();
    for (index, value) in big_num_flags.iter().enumerate() {
        if *value == 1 {
            index_list.push((index + 1).to_string());
        }
    }
    println!("{}", index_list.join(" "))
}
