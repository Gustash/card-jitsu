pub enum Error {
    Serenity(serenity::Error),
    SQLX(sqlx::Error),
    HandleCommand(String),
}
