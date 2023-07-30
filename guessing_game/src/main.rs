use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() 
{
    println!("Guess the number!");

    let secret_nuber = rand::thread_rng().gen_range(1..=100);
    loop 
    {
        println!("Please input your guess.");

        let mut guess= String::new(); //create a new empty string

        io::stdin()
            .read_line(&mut guess) //Input by the command line
            .expect("Failed to read line");  //Enum if return OK indicates the operation was succesfull, ifreturn Err the value of ok is holding and return just the value to you 
            //if you dont use expect the program will compile but you will get a warning
        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_nuber)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
                {
                    println!("You win!");
                    break;
                }
        }
    }
}
