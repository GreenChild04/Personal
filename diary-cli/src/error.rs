pub struct Error {
    origin: String,
    error_type: String,
    msg: String,
}

impl Error {
    pub fn new(origin: &str, error_type: &str, msg: &str) -> Error {
        Error {
            origin: String::from(origin),
            error_type: String::from(error_type),
            msg: String::from(msg),
        }
    }

    pub fn throw(origin: &str, error_type: &str, msg: &str) {
        Self::new(origin, error_type, msg).unwrap();
    }

    pub fn to_string(&self) -> String {
        format!("\x1b[31m[{0}] Error:\x1b[0m {1} \x1b[94m=>\x1b[0m {2}!", self.origin, self.error_type, self.msg)
    }

    fn unwrap(&self) {
        self.panic_unwrap();
        eprintln!("{}", self.to_string());
        std::process::exit(1);
    }

    #[cfg(debug_assertions)]
    fn panic_unwrap(&self) {
        panic!("{}", self.to_string());
    }
}