pub struct Error {
    origin: String,
    error_type: String,
    msg: String,
}

impl Error {
    pub fn new(origin: &str, error_type: &str, msg: String) -> Error {
        Error {
            origin: String::from(origin),
            error_type: String::from(error_type),
            msg: String::from(msg),
        }
    }

    pub fn throw(origin: &str, error_type: &str, msg: String) {
        Self::new(origin, error_type, msg).unwrap();
    }

    pub fn print_err(origin: &str, error_type: &str, msg: String) {
        Self::new(origin, error_type, msg).print();
    }

    pub fn print(self) { eprintln!("{}", self.to_string()); }

    pub fn true_or_throw(self, condition: bool) {
        if !condition { self.unwrap() }
    }

    pub fn true_or_else<F: FnOnce(Self)>(self, condition: bool, f: F) {
        if !condition { f(self) }
    }

    pub fn to_string(&self) -> String {
        format!("\x1b[35;1m[ \x1b[31;1m{0}\x1b[35;1m ] \x1b[31;1mError: \x1b[0m{1}\x1b[35;1m => \x1b[0m{2}\x1b[35;1m!\x1b[0m", self.origin, self.error_type, self.msg)
    }

    pub fn to_raw_string(&self) -> String {
        format!("[ {0} ] Error: {1} => {2}!", self.origin, self.error_type, self.msg)
    }

    fn unwrap(self) {
        self.panic_unwrap();
        self.print();
        std::process::exit(1);
    }

    #[cfg(debug_assertions)]
    fn panic_unwrap(&self) {
        panic!("{}", self.to_raw_string());
    }
}