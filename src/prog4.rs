fn find_first_a(s: &String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }

    None
}

//Find the first occurance of character 'a' in a string
pub fn run() {
   let str: String = String::from("Vishy anand");
   let res: Option<i32> = find_first_a(&str);

   match res {
       Some(index) => println!("Found at index: {}", index),
       None => println!("A not found in the string")
   }
}