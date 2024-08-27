use rand::Rng;
use std::cmp::Ordering;
use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn random_number() -> u8 {
    200
}
fn vairables_and_datatypes() -> () {
    /* SCALAR TYPES */

    //Type annotation for parsing
    let _age: i32 = "24".parse().expect("Not a number");
    let _a: u32 = 20; //unsigned integer takes 32 bit
    let _a: isize = 453; //takes size depend upon system architecture

    /*  n = 8
    i8 -> -(2^n - 1) to 2^(n-1) - 1
    range = -128 to 127
    u8 -> 0 to 2^8 */

    // let _a: u8 = -10; //comialtion error cant store signed value

    //overflowing - literal out of range for `i8`
    //let _a: i8 = -129;
    let _a: i8 = -10; // Correct

    let _b = 255u8; //suffix with the type
    let _b = 255_u8; //suffix with the type
    let _b = 1_00_00_000; //visual seperator for easy to read

    let _b = 0xef; //hexadecimal
    let _b = 0o27; // octal
    let _b = 0b1010; // binary
    let _b = b'A'; // ascii value of A

    /*  Rust includes checks for integer overflow that cause our program to  panic at
        runtime if this behavior occurs. Below program panicked of this reason.
        When compiling using --release flag, Rust does not include checks it.
        Instead, if overflow occurs, Rust performs two’s complement wrapping.
        256 -> 0 257 -> 1 like this.
        So test properly in debug mode, although not crash the program in release.
    */

    //let a = random_number() + 57;
    //println!("Value of a is {a}");  //Value of a is 0 when run using --release flag

    let _a: u8 = random_number().wrapping_add(57); //Not panic in debug also

    let _a: u8 = match random_number().checked_add(55) {
        Some(num) => num,
        None => {
            println!("Cannot add");
            return;
        }
    };

    let (a, b) = random_number().overflowing_add(57);
    println!("Value of a is {a} b is {b}"); // 1 true  <- 1 but overflowed value

    //Floting tyoes are f32 and f64(default). All are signed
    let _c: f32 = 2.12435430943234; // 2.1243543
    let _c: f64 = 2.12435430943234; // 2.12435430943234 - more precision

    let _x: i32 = 5 / 2; // 2
    let _x: f32 = 5_f32 / 2_f32; // 2.5 - floting division
    let _x: f64 = 7.64 / 3.32;

    //4 byte size - Unicode Saclar Value
    let _c: char = 'R';
    let _c: &str = "Rajdip";
    let _heart_eyed_cat = '😻';

    /* COMPOUND TYPES (typles, arrays) */

    //tuple - immutable, can contain different types
    let rajdip: (&str, i32, f64, i32, bool) = ("Rajdip", 22, 6.5, 633, true);
    let (_a, _b, _x, _y, _z) = rajdip;

    println!(
        "1st -> {}, 2nd -> {}, 3rd -> {}, 4th -> {}, 5th -> {}",
        rajdip.0, rajdip.1, rajdip.2, rajdip.3, rajdip.4
    );
    //Empty tuple (unit), return unit value when don't return anything
    let _tup = ();

    //Arrays - fixed length - same type of elements
    let days: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let _x: [i32; 5] = [10; 5]; // x -> [10, 10, 10, 10, 10]

    println!("{}", days[0]); //Sunday

    ();
}

fn shadowing() -> () {
    let apples = 10;
    println!("Value of apple {apples}");

    //the first vairable is shodowed by this one
    let apples = apples + 40;
    println!("Value of apple {apples}");

    /* Using shadowing we can also change the type,
    whereas using mutability we can only change the value */
    let apples: bool = true;
    println!("Value of apple {apples}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");

    println!("{}", THREE_HOURS_IN_SECONDS);

    ();
}

fn guessing_game() -> () {
    println!("!!! GUESSING GAME !!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Input your guess (1 to 100): ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid input: ");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
        }
        println!("Please guess again: ");
    }

    ();
}

fn main() {
    guessing_game();
    shadowing();
    vairables_and_datatypes();
}
