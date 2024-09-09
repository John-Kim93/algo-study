use std::io;

fn main() {
    // 문자열을 저장할 mutable 변수 선언
    let mut tc_input = String::new();

    // 표준 입력에서 한 줄 읽기
    io::stdin().read_line(&mut tc_input).unwrap();

    let test_case: u8 = tc_input.trim().parse().unwrap();
    for _ in 1..=test_case {
        let mut p_input = String::new();
        io::stdin().read_line(&mut p_input).unwrap();

        let mut max_price: u32 = 0;
        let mut answer = String::new();

        // 선수 정보 보기
        let p: u8 = p_input.trim().parse().unwrap();
        for _ in 1..=p {
            let mut info_input = String::new();
            io::stdin().read_line(&mut info_input).unwrap();
            let mut info = info_input.trim().split_whitespace();
            if let Some(val) = info.next() {
                let price: u32 = val.parse().unwrap();
                if price > max_price {
                    max_price = price;
                    if let Some(val) = info.next() {
                        answer = val.replace(" ", "").to_string();
                    }
                }
            }
        }
        println!("{}", answer);
    }
}
