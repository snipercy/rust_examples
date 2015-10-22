use std::io;

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut 表示变量 guess 可变
    // rust变量默认是不可变
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
    
    println!("You guessed {}", guess);
}
