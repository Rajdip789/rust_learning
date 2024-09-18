//Defining the trait
pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("Summarize block"); //default implementation, we can leave this block empty
    }
}

struct User {
    name: String,
    age: u32,
}

//Implementing a trait on the struct, this should implement the summarize fn

//impl Summary for User {} //we can do this also, in this case default fn definition will run

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old.", self.name, self.age);
    }
}

pub fn run() {
    let user = User {
        name: String::from("Rajdip"),
        age: 22,
    };
    println!("{}", user.summarize());
}