pub enum Error {
    Minilink(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            Self::Minilink(text) => "Minilink error: ".to_string() + text + "\n"
        };

        write!(f, "{}", message)
    }
}