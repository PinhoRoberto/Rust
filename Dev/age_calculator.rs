use std:: io;

fn main(){
    println!("Enter your birthday year: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    let birth_year: i32 = input.trim().parse().expect("Enter a valid number");
    let current_year = 2025;
    let age = current_year - birth_year;

    println! ("You are {} years old", age);
}
