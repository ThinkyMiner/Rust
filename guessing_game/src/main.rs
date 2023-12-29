use std::io;// The io(input/output) library is in the std(standard) library...

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();// Here :: means associated function to a type
    //Associated functions are imlpemented on types
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
