use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn parse_u32(string_value: &str) -> u32 {
    string_value.trim().parse::<u32>().expect("Failed to parse input as u32")
}

fn parse_f64(string_value: &str) -> f64 {
    string_value.trim().parse::<f64>().expect("Failed to parse input as f64")
}

fn split_input(string_value: &str) -> Vec<String> {
    string_value
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let tc = parse_u32(&read_input());

    let mut point_stack = 0;
    let mut grade_stack = 0.0;

    for _ in 0..tc {
        let n = parse_u32(&read_input());
        for _ in 0..n {
            if let [point, grade] = &split_input(&read_input())[..] {
                let n_point = parse_u32(point);
                let f_grade = parse_f64(grade);

                point_stack += n_point;
                grade_stack += f_grade * (n_point as f64);
            } else {
                println!("Input does not have exactly 2 values.");
            };
        }
        println!("{} {:.1}", point_stack, grade_stack / (point_stack as f64));
        point_stack = 0;
        grade_stack = 0.0;
    }
}
