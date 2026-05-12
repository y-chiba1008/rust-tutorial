fn main() {
    // if式
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if式から値を返す
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{number}");
}
