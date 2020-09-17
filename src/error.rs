use std::fmt;

pub enum MyCustomError {
    HttpError,
    ParseError,
}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyCustomError::HttpError => write!(f, "HTTP Error"),
            MyCustomError::ParseError => write!(f, "Parse Error")
        }
    }
}

impl From<reqwest::Error> for MyCustomError {
    fn from(_: reqwest::Error) -> Self {
        MyCustomError::HttpError
    }
}

impl From<chrono::format::ParseError> for MyCustomError {
  fn from(_: chrono::format::ParseError) -> Self {
     MyCustomError::ParseError
   }
 }