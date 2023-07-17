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
    let secutiry_level = ask_security_level();

    for index in 0..secutiry_level.length
    {
        let random_char = generate_rand_char(&secutiry_level);
        let character = from_u32(random_char).expect("invalid");
        password.push(character);
    }
    println!("{}", password);
}

fn ask_security_level() -> Options
{
    
    let mut user_option = Options
    {
        length: get_u32_from_stdin("How many length do you want for your password?"),
        uppercase: get_answerd_from_stdin("Do you want uppercase? \n a) Yes \n b) No"),
        lowercase: get_answerd_from_stdin("Do you want lowercase? \n a) Yes \n b) No"),
        numbers: get_answerd_from_stdin("Do you want numbers? \n a) Yes \n b) No"),
        symbols: get_answerd_from_stdin("Do you want symbols? \n a) Yes \n b) No"),
    };
    while user_option.uppercase == 'b' && user_option.lowercase == 'b' && user_option.numbers == 'b' && user_option.symbols == 'b'
    {
        println!("Please select at least one option");
        user_option.uppercase = get_answerd_from_stdin("Do you want uppercase? \n a) Yes \n b) No");
        user_option.lowercase = get_answerd_from_stdin("Do you want lowercase? \n a) Yes \n b) No");
        user_option.numbers = get_answerd_from_stdin("Do you want numbers? \n a) Yes \n b) No");
        user_option.symbols = get_answerd_from_stdin("Do you want symbols? \n a) Yes \n b) No");
        continue;
    }
    
    return user_option;
}

struct Options {
    length: u32,
    uppercase: char,
    lowercase: char,
    numbers: char,
    symbols: char,
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

fn get_answerd_from_stdin(question: &str) -> char
{
    println!("{}", question);
    let mut result: char;
    loop 
    {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        
        match user_input.trim()
        {
            "a" => 
            {
                result = 'a';
                break;
            },
            "yes" => 
            {
                result = 'a';
                break;
            },
            "y" => 
            {
                result = 'a';
                break;
            },
            "b" =>
            {
                result = 'b';
                break;
            },
            "no" =>
            {
                result = 'b';
                break;
            },
            "n" =>
            {
                result = 'b';
                break;
            },
            _ =>
            {
                println!("Please input a valid answerd");
                continue;
            }
        }
    }
    
    return result;
}

fn generate_rand_char(user_input:&Options) -> u32
{
    let mut random_char = 0;
    if user_input.uppercase == 'a' && user_input.lowercase == 'b' && user_input.numbers == 'b' && user_input.symbols == 'b'
    {
        random_char = thread_rng().gen_range(0x41..=0x5A);
    }
    else if user_input.uppercase == 'b' && user_input.lowercase == 'a' && user_input.numbers == 'b' && user_input.symbols == 'b'
    {
        random_char = thread_rng().gen_range(0x61..=0x7A);
    }
    else if user_input.uppercase == 'b' && user_input.lowercase == 'b' && user_input.numbers == 'a' && user_input.symbols == 'b'
    {
        random_char = thread_rng().gen_range(0x30..=0x39);
    }
    else if user_input.uppercase == 'b' && user_input.lowercase == 'b' && user_input.numbers == 'b' && user_input.symbols == 'a'
    {
        
        random_char = thread_rng().gen_range(0x5B..=0x7E);
        while random_char > 0x60 && random_char < 0x7A
        {
            random_char = thread_rng().gen_range(0x5B..=0x7E);
        }
        
    }
    else if user_input.uppercase == 'a' && user_input.lowercase == 'a' && user_input.numbers == 'b' && user_input.symbols == 'b'
    {
        random_char = thread_rng().gen_range(0x41..=0x7A);
        while random_char > 0x5A && random_char < 0x61
        {
            random_char = thread_rng().gen_range(0x41..=0x7A);
        }
    }
    else if user_input.uppercase == 'a' && user_input.lowercase == 'b' && user_input.numbers == 'a' && user_input.symbols == 'b'
    {
        random_char = thread_rng().gen_range(0x30..=0x5A);
        while random_char > 0x39 && random_char < 0x41
        {
            random_char = thread_rng().gen_range(0x30..=0x5A);
        }
    }
    else if user_input.uppercase == 'a' && user_input.lowercase == 'b' && user_input.numbers == 'b' && user_input.symbols == 'a'
    {
        random_char = thread_rng().gen_range(0x3A..=0x7E);
        while random_char > 0x60 && random_char < 0x7B
        {
            random_char = thread_rng().gen_range(0x30..=0x5A);
        }
    }
    else if user_input.uppercase == 'b' && user_input.lowercase == 'a' && user_input.numbers == 'a' && user_input.symbols == 'b'
    {
        random_char = thread_rng().gen_range(0x30..=0x7A);
        while random_char > 0x39 && random_char < 0x61
        {
            random_char = thread_rng().gen_range(0x30..=0x7A);
        }
    }
    else if user_input.uppercase == 'b' && user_input.lowercase == 'a' && user_input.numbers == 'b' && user_input.symbols == 'a'
    {
        random_char = thread_rng().gen_range(0x3A..=0x7E);
        while random_char > 0x40 && random_char < 0x5A
        {
            random_char = thread_rng().gen_range(0x3A..=0x7E);
        }
    }
    else if user_input.uppercase == 'b' && user_input.lowercase == 'b' && user_input.numbers == 'a' && user_input.symbols == 'a'
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