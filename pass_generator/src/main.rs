use std::io;
use rand::{Rng, thread_rng, random};
use std::char::from_u32;

fn main() {
    pass_generator();
}

fn pass_generator()
{
    println!("Welcom to your password generator!");
    let mut password = String::new();
    let secutiry_level = ask_password_options();

    for _index in 0..secutiry_level.length
    {
        let random_char = generate_rand_char(&secutiry_level);
        let character = from_u32(random_char).expect("invalid");
        password.push(character);
    }
    println!("Your password is: {}", password);
}

fn ask_password_options() -> Options
{
    
    let mut user_option = Options
    {
        length: get_u32_from_stdin("How many length do you want for your password?"),
        uppercase: get_answerd_from_stdin("Do you want uppercase? \n a) Yes \n b) No") << 3,
        lowercase: get_answerd_from_stdin("Do you want lowercase? \n a) Yes \n b) No") << 2,
        numbers: get_answerd_from_stdin("Do you want numbers? \n a) Yes \n b) No") << 1,
        symbols: get_answerd_from_stdin("Do you want symbols? \n a) Yes \n b) No") << 0,
        result: 0,
    };

    user_option.result = user_option.uppercase | user_option.lowercase | user_option.numbers | user_option.symbols;

    while user_option.result == 0
    {
        println!("Please select at least one option");
        user_option.uppercase = get_answerd_from_stdin("Do you want uppercase? \n a) Yes \n b) No");
        user_option.lowercase = get_answerd_from_stdin("Do you want lowercase? \n a) Yes \n b) No");
        user_option.numbers = get_answerd_from_stdin("Do you want numbers? \n a) Yes \n b) No");
        user_option.symbols = get_answerd_from_stdin("Do you want symbols? \n a) Yes \n b) No");
        user_option.result = user_option.uppercase | user_option.lowercase | user_option.numbers | user_option.symbols;
        continue;
    }
    
    return user_option;
}

struct Options {
    length: u32,
    uppercase: u8,
    lowercase: u8,
    numbers: u8,
    symbols: u8,
    result: u8,
}

fn get_u32_from_stdin(question: &str) -> u32
{
    println!("{}", question);
    let mut result = 0;
    loop 
    {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input: u32 = match user_input.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => 
            {
                println!("Please input a value grater than 0");
                continue;
            }
        };
        if user_input > 0
        {
            result = user_input;
            break;
        }
        else 
        {
            println!("Please input a value grater than 0");
            continue;
        }
    }
    
    return result;
}

fn get_answerd_from_stdin(question: &str) -> u8
{
    println!("{}", question);
    let mut result: u8;
    loop 
    {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        
        if user_input.trim() == "a" || user_input.trim() == "yes" || user_input.trim() == "y"
        {
            result = 1;
            break;
        } 
        else if user_input.trim() == "b" || user_input.trim() == "no" || user_input == "n"
        {
            result = 0;
            break;
        }
        else 
        {
            println!("Please input a valid answerd");
            continue;
        }
    }
    
    return result;
}

fn generate_rand_char(user_input:&Options) -> u32
{
    let mut random_char = 0;
    if user_input.result == 0b1000
    {
        random_char = thread_rng().gen_range(0x41..=0x5A);
    }
    else if user_input.result == 0b0100
    {
        random_char = thread_rng().gen_range(0x61..=0x7A);
    }
    else if user_input.result == 0b0010
    {
        random_char = thread_rng().gen_range(0x30..=0x39);
    }
    else if user_input.result == 0b0001
    {
        
        random_char = thread_rng().gen_range(0x5B..=0x7E);
        while random_char > 0x60 && random_char < 0x7A
        {
            random_char = thread_rng().gen_range(0x5B..=0x7E);
        }
        
    }
    else if user_input.result == 0b1100
    {
        random_char = thread_rng().gen_range(0x41..=0x7A);
        while random_char > 0x5A && random_char < 0x61
        {
            random_char = thread_rng().gen_range(0x41..=0x7A);
        }
    }
    else if user_input.result == 0b1010
    {
        random_char = thread_rng().gen_range(0x30..=0x5A);
        while random_char > 0x39 && random_char < 0x41
        {
            random_char = thread_rng().gen_range(0x30..=0x5A);
        }
    }
    else if user_input.result == 0b1001
    {
        random_char = thread_rng().gen_range(0x3A..=0x7E);
        while random_char > 0x60 && random_char < 0x7B
        {
            random_char = thread_rng().gen_range(0x30..=0x5A);
        }
    }
    else if user_input.result == 0b0110
    {
        random_char = thread_rng().gen_range(0x30..=0x7A);
        while random_char > 0x39 && random_char < 0x61
        {
            random_char = thread_rng().gen_range(0x30..=0x7A);
        }
    }
    else if user_input.result == 0b0101
    {
        random_char = thread_rng().gen_range(0x3A..=0x7E);
        while random_char > 0x40 && random_char < 0x5A
        {
            random_char = thread_rng().gen_range(0x3A..=0x7E);
        }
    }
    else if user_input.result == 0b0011
    {
        random_char = thread_rng().gen_range(0x30..=0x7E);
        while random_char > 0x40 && random_char < 0x5A || random_char > 0x60 && random_char < 0x7B
        {
            random_char = thread_rng().gen_range(0x3A..=0x7E);
        }
    }
    else 
    {
        random_char = thread_rng().gen_range(0x30..=0x7E);
    }

    return random_char;
}