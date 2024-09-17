use chrono::{Local, Utc};

pub fn run() {
    let now = Utc::now();
    println!("Current data and time in UTC: {}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    let local = Local::now();
    println!("Current date and time in local: {}", local);
}