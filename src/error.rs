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