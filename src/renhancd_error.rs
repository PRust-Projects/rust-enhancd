use colored::*;
use std::process::exit;

pub fn error(message: &str) {
    println!("{}", message.red());
}

pub fn error_and_exit(message: &str) -> ! {
    error(message);
    exit(1)
}

pub trait ErrorExt<T> {
    fn exit_if_err(self, message: &str);
    fn unwrap_or_error_and_exit(self, message: &str) -> T;
}

impl<T, E> ErrorExt<T> for Result<T, E> {
    fn exit_if_err(self, message: &str) {
        if self.is_err() {
            error_and_exit(message);
        }
    }

    fn unwrap_or_error_and_exit(self, message: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => error_and_exit(message),
        }
    }
}

impl<T> ErrorExt<T> for Option<T> {
    fn exit_if_err(self, message: &str) {
        if self.is_none() {
            error_and_exit(message);
        }
    }

    fn unwrap_or_error_and_exit(self, message: &str) -> T {
        match self {
            Some(t) => t,
            None => error_and_exit(message),
        }
    }
}
