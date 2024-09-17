fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }

    return  false;
}

//Check a number is even or odd
pub fn run() {
    let num: i32 = 24;
    println!("{}",is_even(num));
}