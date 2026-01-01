use std::io;

fn main() {
    println!("Enter your year of birth:");
    let mut yob = String::new();

    io::stdin()
        .read_line(&mut yob)
        .expect("Failed to read line");

    // Parse the string to u64
    let yob: u64 = yob.trim().parse().expect("Please enter a valid number");

    // Calculate age
    let age = 2026 - yob;

    println!("Your age is {} years", age);
}

//version 2 using a struct

// struct AgeCalculator {
//     current_year: u64,
// }

// impl AgeCalculator {
//     fn new() -> Self {
//         AgeCalculator { current_year: 2026 }
//     }

//     fn calculate_age(&self, year_of_birth: u64) -> u64 {
//         self.current_year - year_of_birth
//     }
// }

// fn main() {
//     println!("Enter your year of birth:");
//     let mut yob = String::new();

//     io::stdin()
//         .read_line(&mut yob)
//         .expect("Failed to read line");

//     let yob: u64 = yob.trim().parse().expect("Please enter a valid number");

//     let calculator = AgeCalculator::new();
//     let age = calculator.calculate_age(yob);

//     println!("Your age is {} years", age);
// }
