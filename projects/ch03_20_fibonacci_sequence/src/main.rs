use std::io;

fn main() {
    let n = input_number();
    let value = fibonacci_sequence(n);
    println!("f({n}) = {value}");
}

fn input_number() -> u32 {
    loop {
        // 回答を入力
        println!("Plese input n (n >= 0)");
        let mut input_val = String::new();
        io::stdin()
            .read_line(&mut input_val)
            .expect("Failed to read line.");

        // 型変換
        break match input_val.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
    }
}

fn fibonacci_sequence(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2)
    }
}
