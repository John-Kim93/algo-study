use std::io;

fn main() {
    // 문자열을 저장할 mutable 변수 선언
    let mut min_input = String::new();
    let mut max_input = String::new();

    // 표준 입력에서 한 줄 읽기
    io::stdin().read_line(&mut min_input).unwrap();
    io::stdin().read_line(&mut max_input).unwrap();
    let min: i32 = min_input.trim().parse().unwrap();
    let max: i32 = max_input.trim().parse().unwrap();

    let mut answer: Vec<i32> = Vec::new();

    for i in 1..=100 {
        let cur_val = i * i;
        if cur_val >= min && cur_val <= max {
            answer.push(cur_val);
        }
    }
    if answer.is_empty() {
        println!("{}", -1);
        return;
    }
    let sum: i32 = answer.iter().sum();
    println!("{}", sum);
    println!("{}", answer[0]);
    return;
}
