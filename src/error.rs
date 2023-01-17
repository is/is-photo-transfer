#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

#[derive(Debug)]
struct MyError {
    errno: i32,
    details: String
}

impl MyError {

    fn new(errno:i32, msg: &str) -> MyError {
        MyError {errno, details:msg.to_string()}
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}


pub type MyResult<T> = Result<T, Box<dyn Error>>;