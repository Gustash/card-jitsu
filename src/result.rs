pub enum Error {
    Serenity(serenity::Error),
    SQLX(sqlx::Error),
    HandleCommand(String),
    ExistingActiveChallenge(u64),
}

impl From<serenity::Error> for Error {
    fn from(error: serenity::Error) -> Self {
        Error::Serenity(error)
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::SQLX(error)
    }
}
