use std::io;

fn main() {
    // 문자열을 저장할 mutable 변수 선언
    let mut input_01 = String::new();

    // 표준 입력에서 한 줄 읽기
    io::stdin().read_line(&mut input_01).unwrap();
    let max_count = input_01.trim().parse::<i32>().unwrap();
    let mut i = 1;
    let mut change = 1;

    let mut output: String = String::new();
    while i != 0 {
        let mut j = 0;
        while j != max_count - i {
            output.push_str(" ");
            j += 1;
        }
        for _ in 0..i {
            output.push_str("*");
        }
        if i == max_count {
            change = -1;
        }
        output.push_str("\n");
        i += change;
    }
    println!("{}", output);
}
