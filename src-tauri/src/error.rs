pub enum Error {
    SomeError
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            Self::SomeError => "Some error"
        };

        write!(f, "{}", message)
    }
}