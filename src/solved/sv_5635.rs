use std::io;

#[derive(Clone)]
struct Student {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

impl Student {
    fn new(name: String, day: i32, month: i32, year: i32) -> Self {
        Student { name, day, month, year }
    }
}
fn main() {
    // 문자열을 저장할 mutable 변수 선언
    let mut input_01 = String::new();

    // 표준 입력에서 한 줄 읽기
    io::stdin().read_line(&mut input_01).unwrap();
    let student_cnt = input_01.trim().parse::<i32>().unwrap();

    let mut big_boy: Student = Student::new("".to_string(), 0, 0, 0);
    let mut mini_boy: Student = Student::new("".to_string(), 0, 0, 0);

    for _ in 0..student_cnt {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let (name, day, month, year): (String, i32, i32, i32) = {
            let mut split = input.split_whitespace();
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        };

        let a: Student = Student::new(name, day, month, year);

        if big_boy.day == 0 {
            big_boy = a.clone();
            mini_boy = a.clone();
            continue;
        }

        if check_bigger(a.clone(), big_boy.clone()) {
            big_boy = a.clone();
        }
        if !check_bigger(a.clone(), mini_boy.clone()) {
            mini_boy = a.clone();
        }
    }
    println!("{}", mini_boy.name);
    println!("{}", big_boy.name);
}

fn check_bigger(student_a: Student, target: Student) -> bool {
    if student_a.year > target.year {
        false
    } else if student_a.year < target.year {
        true
    } else {
        if student_a.month > target.month {
            false
        } else if student_a.month < target.month {
            true
        } else {
            if student_a.day > target.day {
                false
            } else {
                true // day까지 같은 케이스는 없음
            }
        }
    }
}
