use std::error;
use std::result;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;