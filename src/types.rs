use std::fmt;

pub use ggez::nalgebra as na;


#[derive(Debug)]
pub enum Error {
    GgezError(ggez::GameError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::GgezError(ref e) => write!(f, "ggez error: {}", e),
        }
    }
}