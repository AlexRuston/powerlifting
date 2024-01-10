mod lifter;

use std::io;
use lifter::Lifter;

fn main() {
    // get bodyweight
    println!("Enter your bodyweight (kg)...");
    let bodyweight = get_input();

    // get squat
    println!("Enter your max squat (kg)...");
    let squat = get_input();

    // get bench
    println!("Enter your max bench (kg)...");
    let bench = get_input();

    // get deadlift
    println!("Enter your max deadlift (kg)...");
    let deadlift = get_input();

    let powerlifter = Lifter::new(
        bodyweight,
        squat,
        bench,
        deadlift,
    );
    
    println!("Those maxes give you a total of: {}kg, with a Wilks of: {}", powerlifter.total(), powerlifter.wilks());
}

fn get_input() -> f32 {
    let mut input_text = String::new();
    
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");
        
    let input_return: f32 = input_text.trim().parse().ok().expect("That's no number!");

    input_return
}