use std::io;
use std::io::Write;
use std::fmt::{Display, self};
use crate::error;

pub enum Log<'a> {
    /// Basic Owned String
    /// ```pseudo
    /// ( String )
    /// ```
    Raw(String),
    /// Basic String Slice
    /// ```pseudo
    /// ( &str )
    /// ```
    Str(&'a str),
    /// Default base of every normal log
    /// ```pseudo
    /// ( origin: &str, title: &str, body: &Log )
    /// stdout = "[ " + origin + " ] " + title + " => " + body
    /// ```
    Base(&'a str, &'a str, &'a Log<'a>),
    /// Wraps body with string slices
    /// ```pseudo
    /// ( left: &str, body: &Log, right: &str )
    /// stdout = left + " " + body + " " + right
    /// ```
    Wrap(&'a str, &'a Log<'a>, &'a str),
    /// Separates two elements with a string slice
    /// ```pseudo
    /// ( left: &Log, sep: &str, right: &Log )
    /// stdout = left + sep + right
    /// ```
    List(&'a Log<'a>, &'a str, &'a Log<'a>),
    /// Creates a wrap around an string slice ( commonly used as the origin of a log )
    /// ```pseudo
    /// ( body: &str )
    /// stdout = "[ " + body + " ]"
    /// ```
    Origin(&'a str),
    /// Makes text blue
    /// ```pseudo
    /// ( &str )
    /// ```
    Title(&'a str),
    /// Makes text red and bold
    /// ```pseudo
    /// ( &str )
    /// ```
    Red(&'a str),
    /// Does absolutely nothing, just a place-holder
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
                Base(origin, title, body) => format!( "{0} {1} {2} {3}", Origin(origin), title, Self::blue(">>"), body),
                List(x, sep, y) => format!( "{0}{1}{2}", x, Self::blue(sep), y),
                Origin(x) => format!("{}", Wrap("[", &Title(x), "]")),
            }
        )
    }
}

impl<'a> Log<'a> {
    pub fn log(self) {
        println!("{}", self);
    }

    // colours
    fn colour(log: &str, colour: &str) -> String {
        format!( "{colour}{log}\x1b[0m" )
    }

    fn green(log: &str) -> String { Self::colour(log, "\x1b[32m") }
    fn blue(log: &str) -> String { Self::colour(log, "\x1b[34;1m") }
    fn red(log: &str) -> String { Self::colour(log, "\x1b[31;1m") }
}

/// Logs to stdout
/// ```pseudo
/// stdout = "[ " + origin + " ] " + title + " => " + body
/// ```
pub fn log(origin: &str, title: &str, body: &str) {
    use Log::*;
    Base(origin, title, &Str(body)).log();
}

/// Takes in user input
/// ```pseudo
/// stdout = "[ " + origin + " ] " + prompt + ": " + stdin
/// ```
pub fn input(origin: &str, prompt: &str) -> String {
    let mut user: String = String::new();
    print!("{}", {
        use Log::*;
        List(
            &Origin(origin), " ",
            &List(&Str(prompt), " << ", &Void),
        )
    });
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user).expect("Failed to read line");
    user
}


/// Takes in user input ( only yes or no )
/// ```pseudo
/// stdout = "[ " + origin + " ] " + prompt + " [y/n]: " + stdin
/// ```
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
            println!("{}", error::init::<String>(origin, "Invalid Input", &format!("Error expected 'yes' or 'no' not '{yesno}'")).crash());
            input_yes_no(origin, prompt)
        },
    }
}