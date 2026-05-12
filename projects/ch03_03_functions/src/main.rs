fn main() {
    println!("Hello, world!");

    let x = another_function(5);
    println!("The return value is: {x}");
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is: {x}");
    x * 2
}
