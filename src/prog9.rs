//Write a function that takes a string as an input and returns the first word from it

fn first_word(name: &str) -> &str {
    let mut ind: usize = 0;

    for ch in name.chars() {
        if ch == ' ' {
            return &name[0..ind];
        }
        ind += 1;
    }

    &name[..]
}

pub fn run() {
    let name: String = String::from("hello world");
    let ans: &str = first_word(&name);
    println!("First word in the string: {}", ans);
}