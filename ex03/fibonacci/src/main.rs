use std::io;

fn main() {
    println!("n 번째 피보나치 수를 구합니다.");
    let mut buffer: String = String::new();
    io::stdin().read_line(& mut buffer)
        .expect("값을 입력받는데 실패하였습니다.");

    let n: u32 = buffer.trim().parse()
        .expect("숫자값을 입력해주세요.");

    println!("result: {}", fibonacci(n))
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => {
            let result = fibonacci(n-1) + fibonacci(n-2);
            result
        }
    }
}
