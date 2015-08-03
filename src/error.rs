/// Errors for use with Switchboard.

use serde::json;
use std::error;
use std::fmt;
use std::io;
use std::result;
use std::sync::mpsc;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Disconnected(mpsc::RecvError),
    JsonParsing(json::error::Error),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind: kind,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(&*self).fmt(f)
    }
}

impl error::Error for Error {
  fn description(&self) -> &str {
      match self.kind {
          ErrorKind::Io(ref e) => e.description(),
          ErrorKind::Disconnected(_) => "Channel is disconnected.",
          ErrorKind::JsonParsing(ref e) => e.description(),
      }
  }

  fn cause(&self) -> Option<&error::Error> {
      match self.kind {
          ErrorKind::Io(ref e) => Some(e),
          ErrorKind::Disconnected(ref e) => Some(e),
          ErrorKind::JsonParsing(ref e) => Some(e),
      }
  }
}

impl From<io::Error> for Error {
     fn from(error: io::Error) -> Self {
         Error::new(ErrorKind::Io(error))
     }
}

impl From<mpsc::RecvError> for Error {
     fn from(error: mpsc::RecvError) -> Self {
         Error::new(ErrorKind::Disconnected(error))
     }
}

impl From<json::error::Error> for Error {
     fn from(error: json::error::Error) -> Self {
         Error::new(ErrorKind::JsonParsing(error))
     }
}