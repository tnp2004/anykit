pub enum Error {
    InvalidInput(String),
    Other,
    Generate(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            Self::InvalidInput(e) => e,
            Self::Other => "Something went wrong!. Try again!",
            Self::Generate(e) => e,
        };

        write!(f, "{}", message)
    }
}