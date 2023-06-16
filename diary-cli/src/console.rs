pub mod error;
use std::io;
use std::io::Write;

pub fn input(origin: &str, prompt: &String) -> String {
    let mut user: String = String::new();
    print!("\x1b[35;1m[ \x1b[34m{origin}\x1b[35;1m ] \x1b[34m{prompt}\x1b[35;1m:\x1b[0m ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user).expect("Failed to read line");
    return user;
}

pub fn input_yes_no(origin: &str, prompt: &str) -> bool {
    let yesno: String = input(origin, &format!("{prompt} \x1b[35;1m[\x1b[34my\x1b[35;1m/\x1b[34mn\x1b[35;1m]")).to_lowercase();
    let yesno: &str = yesno.trim();
    match yesno {
        "yes" => true,
        "no" => false,
        "y" => true,
        "n" => false,
        _ => {
            error::Error::print_err(&origin, "Invalid Input", format!("Error expected 'yes' or 'no' not '{yesno}'"));
            input_yes_no(origin, prompt) 
        },
    }
}