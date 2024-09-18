use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::{fs, io};

mod prog1;
mod prog2;
mod prog3;
mod prog4;
mod prog5;
mod prog6;
mod prog7;
mod prog8;
mod prog9;
mod prog10;
mod structs;
mod traits;
mod iterators;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn slices () {
    // 3 types of strings
    let name = String::from("hello world"); //String type
    let string_slice = &name; // Has a 'view' into the original string/ is a refernce
    let string_literal = "hello"; // Literal is also an &str but it points directly to an address in the binary

    // slice can also be applied to other collections like vectors/arrays
    let arr = [1,2,3];
    let arr_slice = &arr[0..1];
}

//Result enum written in rust
/* enum Result<A, B> {
    Ok(A),
    Err(B),
} */

//Option enum
/* enum Option<T> {
    Some(T),
    None,
} */

fn error_handling() {
    let s = "Rajdip";
    let res: Result<String, io::Error> = fs::read_to_string("readme.md");

    match res {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

struct Point<A, B> {
    x: A,
    y: A,
    z: B,
}

//use when we have limited values of something
enum Direction {
    North,
    East,
    South,
    West,    
}
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    //pattern matching
    let area = match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length, 
        Shape::Rectangle(width, height) => width * height,
    };

    area
}

fn enums() {
    let my_dir = Direction::North;
    let new_dir = my_dir;

    let circle = Shape::Circle(5.2);
    let rectangle = Shape::Rectangle(20.0, 30.0);

    println!("Area of the circle: {}", calculate_area(circle));
    println!("Area of the reactangle: {}", calculate_area(rectangle));

    let floting_point: Point<f64, &str> = Point  {x: 5.0, y: 4.5, z: "12" };
    println!("Floting Point: ({}, {})", floting_point.x, floting_point. y);
}

struct NoShape; //unit struct

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

struct  User {
    username: String,
    email: String,
    active: bool,
    sing_in_count: u64,
}

fn structure() {
    let user1 = User {
        username: String::from("Rajdip Pal"),
        email: String::from("palrajdip33@gmail.com"),
        active: true,
        sing_in_count: 1,
    };
    println!("Name: {}, Email: {}, Is active: {}, Sign in count: {}", user1.username, user1.email, user1.active, user1.sing_in_count);

    let rect1= Rect {
        width: 52,
        height: 40
    };

    println!("Height: {}", rect1.width);
    println!("Width: {}", rect1.height);
    println!("Area: {}", rect1.area());
    println!("Perimeter: {}", rect1.perimeter());
}

fn references() {
    /*  pass a reference to the fucntion insted of moving / transfering the ownership to the function
        A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; 
        that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value 
        of a particular type for the life of that reference. */

    /*  The Rules of References
        1.  At any given time, you can have either one mutable reference or any number of immutable references.
        2.  References must always be valid. */

    //Non mutable reference  
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    //Mutable reference
    let mut s2 = String::from("Hello");
    modify_str(&mut s2);

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;

    //Problem - cannot borrow `s` as mutable because it is also borrowed as immutable
    //let r3 = &mut s;
    println!("{r1} and {r2}");

    //No problem - The scopes of the immutable references r1 and r2 end after the println! where they are last used.
    let r3 = &mut s;
    println!("{r3}");

    //dangling reference 
    //let reference_to_nothing = dangle();

}

//Can't return the reference, because the scope of s is end after this function
// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s  //we can return the actual s, insted of the ref, to transfer ownership
// }

fn modify_str(s2: &mut String) {
    s2.push_str(", World");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

//https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
fn ownership () {
    /*  1. string literal -> we know the contents at compile time, so the text is hardcoded 
        directly into the final executable. This is why string literals are fast and efficient. 
        But these properties only come from the string literal’s immutability. 

        2. String type -> in order to support a mutable, growable piece of text, we need to allocate 
        an amount of memory on the heap, unknown at compile time, to hold the contents.
            1. The memory must be requested from the memory allocator at runtime. (String::from)
            2. We need a way of returning this memory to the allocator when we’re done with our String.
    */
    let mut str = String::from("");
    str.push_str("This is some piece of text");

    println!("Pointer: {:p}, length: {}, capacity: {}", str.as_ptr(), str.len(), str.capacity());

    /*  we copy the pointer, the length, and the capacity that are on the stack. 
        We do not copy the data on the heap that the pointer refers to. It it known as move.
        we called -> str was moved into s */
    let _s = str;        //after this str is no longer valid
    //println!("{str}, world!");  //so this line will give error

    // Deep copy - copies the actual heap data - clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    /*  The mechanics of passing a value to a function are similar to those when assigning a value to a variable. 
    Passing a variable to a function will move or copy, just as assignment does. */
    takes_ownership(s1);             // Transfering ownership -> s's value moves into the function...
                                    // ... and so is no longer valid here

    //println!("{s1}"); //Invlid because , s1 moves into the takes_ownership function

}

fn control_flow(y: bool) {

    //Type annotation is must in function definition

    /*  Statements: are instructions that perform some action and do not return a value. fucntion declaration

        Expressions: evaluate to a resultant value. Expressiion can be part of statements.
        Macro calling, function calling
    */

    //if expression
    let x: i32 = if y { 10 } else { 20 };
    println!("x = {x}");

    let _number = 3;

    /* Error -> In rust if condition always expects a boolean value, it will not 
        convert a non boolean to boolean by default like other languages
    */
    // if number {
    //     println!("The number is three");
    // }

    //Loop - breaking conditioin not known
    let mut num: u8 = 1;
    loop {
        println!("Vaue of number is {num}");
        
        /* Infinity loop, will get overflow and crash - but in release mode it will break after 128 
        because of wrapping the (128 + 128) = 256 -> 0 */
        if num == 0 {
            break;
        }
        num = num + num;
    }

    let mut num = 1;
    let _result: i32 = loop { //loop as expression
        if num == 10 { 
            break num;
        }
        num = num + 1;
    };


    //Loop label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while looop
    let mut number = 0;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let arr = [1, 2, 3, 4, 5, 6];
    let mut index = 0;

    while index < arr.len() {
        println!("index: {}, value: {}", index, arr[index]);
        index += 1;
    }

    //efficient approach
    for e in arr {
        println!("value: {e}");
    }

    for number in (1..=10).rev() { //To make last index inclusive put = sign after ..
        println!("{number}");
    }
}

fn function_example(x: i32, y: i32) -> i32 {
    if x == 0 || y == 0 {
        return 0; //early return
    }
    println!("Value of x: {x}");
    println!("Value of y: {y}");

    let y = {
        let x = x+y;
        x + 1 //return expression, not include ending semicolon
    };

    println!("The value of y is {y}");

    y //return
}

fn random_number() -> u8 {
    200 //return 200
}

fn vairables_and_datatypes() {
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
    //string slice initialized with a string literal. String literals have a static lifetime
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

}

fn shadowing() {
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
}

fn guessing_game() {
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
}

fn main() {
    // guessing_game();
    // shadowing();
    // vairables_and_datatypes();
    // let _x = function_example(5, 3);
    // control_flow(true);

    // ownership();
    // references();

    // structure();
    // enums();

    // error_handling();

    prog1::run();
    prog2::run();
    prog3::run();
    prog4::run();
    prog5::run();
    prog6::run();
    prog7::run();
    prog8::run();
    prog9::run();
    prog10::run();

    structs::run();
    traits::run();
    iterators::run();
}
