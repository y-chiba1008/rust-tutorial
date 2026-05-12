fn main() {
    loop_returns_value();
    loop_label();
    while_statement();
    for_in_collection();
    for_in_range();
}

fn loop_returns_value() {
    println!("== loop_returns_value ========");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // breakの後ろに返す値を書く
            break counter * 2;
        }
    };
    println!("result: {result}");
    println!();
}

fn loop_label() {
    // ループラベル
    println!("== loop_label ========");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // 内側のloopを抜ける
                break;
            }
            if count == 2 {
                // ラベルを指定して外側のloopを抜ける
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!();
}

fn while_statement() {
    println!("== while ========");
    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!();
}

fn for_in_collection() {
    println!("== for_in_collection ========");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!();
}

fn for_in_range() {
    println!("== for_in_range ========");
    // .rev()で逆順
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    println!();
}
