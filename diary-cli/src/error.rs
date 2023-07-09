use std::fmt::Debug;
use crate::Log;

pub struct ErrorData<T> {
    origin: String,
    error_type: String,
    msg: String,
    result: Option<T>,
}

pub fn from<T, E>(result: Result<T, E>, origin: &str, error_type: &str, msg: String) -> ErrorData<T>
where
    E: Debug,
{
    ErrorData {
        origin: String::from(origin),
        error_type: String::from(error_type),
        msg,
        result: if result.is_ok() { Some(result.unwrap()) } else { None }
    }
}

pub fn init<T>(origin: &str, error_type: &str, msg: &str) -> ErrorData<T> {
    ErrorData {
        origin: String::from(origin),
        error_type: String::from(error_type),
        msg: String::from(msg),
        result: None,
    }
}

impl<T> ErrorData<T> {
    pub fn to_string(&self, fatal: bool) -> String {
        use Log::*;
        List(
            &List(
                &List(
                    &Origin(&self.origin), " ",
                    &Red(if fatal { "Fatal" } else { "Error" })
                ), ": ",
                &Str(&self.error_type),
            ), " >> ",
            &Str(&self.msg),
        ).to_string()
    }

    pub fn retry<F: Fn() -> ErrorData<T>>(self, idx: u8, f: F) -> T { // handles error
        if self.result.is_some() { return self.result.unwrap() }
        if idx < 1 { return self.crash() }
        println!("{}", self.to_string(false));
        crate::log(&self.origin, "Error Recovery", "Retrying action...");
        f().retry(idx - 1, f)
    }

    pub fn true_or_throw(self, condition: bool) { // Consumes error
        if !condition { self.crash(); }
    }

    pub fn true_or_else<F: FnOnce(Self)>(self, condition: bool, f: F) { // Consumes error
        if !condition { f(self) }
    }

    pub fn to_raw_string(&self, fatal: bool) -> String {
        format!("[ {0} ] {1}: {2} >> {3}", self.origin, if fatal { "Fatal" } else { "Error" }, self.error_type, self.msg)
    }

    pub fn crash(self) -> T { // Handles error
        if self.result.is_some() { return self.result.unwrap() }
        self.debug_crash(); // should only run during debug
        println!("{}", self.to_string(true));
        std::process::exit(1);
    }

    #[cfg(debug_assertions)]
    fn debug_crash(&self) { // Panics instead for traceback
        panic!("{}", self.to_raw_string(true));
    }

}