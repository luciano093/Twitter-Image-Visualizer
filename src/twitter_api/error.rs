#[derive(Debug)]
pub enum Error {
    NoUserFound,
    UserUnavailable,
    BadToken
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NoUserFound => "no user was found",
            Self::UserUnavailable => "user is unavailable",
            Self::BadToken => "bad token",
        };

        write!(f, "Error: {}", msg)
    }
}

impl std::error::Error for Error { }