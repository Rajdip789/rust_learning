fn get_string_length(str : &str) -> usize {
    str.chars().count()
}

//Print number of characters in a string
pub fn run() {
    let str1 = "Rajdip Pal";
    let str2 = String::from("Sandip");

    println!("No of characters in the string: {}", get_string_length(str1));
    println!("No of characters in the string: {}", get_string_length(&str1[7..]));
    println!("No of characters in the string: {}", get_string_length(&str2));
}