use std::io;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    
    loop{
         println!("Please input your guess.");

        // mut 表示变量 guess 可变
        // rust变量默认是不可变
        let mut guess = String::new();

        let secret_num= 5;

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");
     
        // haha
        // trim(),eliminate any white space
        // parse(),parse a string into other a number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) =>  num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_num){
            Ordering::Less     => println!("Small"),
            Ordering::Greater  => println!("big"),
            Ordering::Equal    => {
                println!("You Win");
                break;
            }
        }
    }
}
