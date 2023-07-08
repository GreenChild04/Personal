pub mod error;
use std::io;
use std::io::Write;
use std::fmt::{Display, self};

pub enum Log<'a> {
    Raw(String), // basic string
    Str(&'a str), // basic str
    Base(&'a str, &'a str, &'a Log<'a>), // Base of every log ( tick, title, body )
    Wrap(&'a str, &'a Log<'a>, &'a str), // wrapping eg. '['value']'
    List(&'a Log<'a>, &'a str, &'a Log<'a>), // List with separator
    Origin(&'a str), // Origin of a log
    Title(&'a str), // Blue title
    Red(&'a str), // Red Error
    Void,
}

impl<'a> Display for Log<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Log::*;
        write!(
            f,
            "{}",
            match self {
                // basic
                Raw(x) => x.clone(),
                Str(x) => x.to_string(),
                Title(x) => Self::green(x),
                Red(x) => Self::red(x),
                Void => String::new(),

                // not basic
                Wrap(a, log, b) => format!( "{0} {1} {2}", Self::blue(a), log, Self::blue(b) ),
                Base(origin, title, body) => format!( "{0} {1} {2} {3}", Origin(origin), Title(title), Self::blue("=>"), body),
                List(x, sep, y) => format!( "{0}{1}{2}", x, Self::blue(sep), y),
                Origin(x) => format!("{}", Wrap("[", &Title(x), "]")),
            }
        )
    }
}

impl<'a> Log<'a> {
    pub fn log(self) {
        println!("{}", self.to_string());
    }

    // colours
    fn colour(log: &str, colour: &str) -> String {
        format!( "{colour}{log}\x1b[0m" )
    }

    fn green(log: &str) -> String { Self::colour(log, "\x1b[32m") }
    fn blue(log: &str) -> String { Self::colour(log, "\x1b[34;1m") }
    fn red(log: &str) -> String { Self::colour(log, "\x1b[31;1m") }
}

pub fn log(origin: &str, title: &str, body: String) {
    use Log::*;
    Base(origin, title, &Raw(body)).log();
}

pub fn input(origin: &str, prompt: &str) -> String {
    let mut user: String = String::new();
    print!("{}", {
        use Log::*;
        List(
            &Origin(origin), " ",
            &List(&Str(prompt), ": ", &Void),
        )
    });
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user).expect("Failed to read line");
    return user;
}

pub fn input_yes_no(origin: &str, prompt: &str) -> bool {
    let yesno: String = input(origin, {
        use Log::*;
        &List(
            &Str(prompt), " ",
            &Wrap("[", &List(
                &Title("y"), "/",
                &Title("n"),
            ), "]"),
        ).to_string()
    });
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