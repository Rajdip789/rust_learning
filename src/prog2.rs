fn fibo(num: i32) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 1;

    if num < 2 {
        return num;
    }

    for _ in 0..num-2 {
        let temp: i32 = first + second;
        first = second;
        second = temp;
    }

    return  second;
}
// 0 1 2 3 4 5 6 7
// 0 1 1 2 3 5 8 13
// 1 2 3 4 5 6 7 8

//Print nth fibonacci number
pub fn run() {
    let num = 7;
    println!("{}", fibo(num));
}