use super::models::{Challenge, ChallengeStatus};
use sqlx::{sqlite::SqliteDone, SqlitePool};
use std::env;

pub async fn connect_to_db() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url)
        .await
        .expect(&format!("Error connecting to {}", database_url))
}

pub async fn create_challenge(
    conn: &SqlitePool,
    challenger_id: &u64,
    challenged_id: &u64,
) -> Result<SqliteDone, sqlx::Error> {
    let challenger_id_str = challenger_id.to_string();
    let challenged_id_str = challenged_id.to_string();

    sqlx::query!(
        "INSERT INTO challenges (challenger, challenged) VALUES (?, ?)",
        challenger_id_str,
        challenged_id_str,
    )
    .execute(conn)
    .await
}

pub async fn find_challenge(
    conn: &SqlitePool,
    challenger_id: &u64,
    challenged_id: &u64,
) -> Result<Challenge, sqlx::Error> {
    let challenger_id_str = challenger_id.to_string();
    let challenged_id_str = challenged_id.to_string();

    sqlx::query_as!(
        Challenge,
        "SELECT id, challenger, challenged, accepted, winner FROM challenges WHERE challenger = ? AND challenged = ?",
        challenger_id_str,
        challenged_id_str,
    )
    .fetch_one(conn)
    .await
}

// TODO: Look into replacing Future with Stream
pub async fn find_all_challenges(
    conn: &SqlitePool,
    user_id: &u64,
) -> Result<Vec<Challenge>, sqlx::Error> {
    let user_id_str = user_id.to_string();

    sqlx::query_as!(
        Challenge,
        "SELECT id, challenger, challenged, accepted, winner FROM challenges WHERE challenger = ?1 OR challenged = ?1",
        user_id_str,
    )
    .fetch_all(conn)
    .await
}

pub async fn find_challenges(
    conn: &SqlitePool,
    user_id: &u64,
    status: ChallengeStatus,
) -> Result<Vec<Challenge>, sqlx::Error> {
    let user_id_str = user_id.to_string();
    let status_bool = match status {
        ChallengeStatus::Pending => "FALSE",
        ChallengeStatus::Ongoing => "TRUE",
    };

    sqlx::query_as!(
        Challenge,
        "SELECT id, challenger, challenged, accepted, winner FROM challenges WHERE (challenger = ?1 OR challenged = ?1) AND accepted = ?2",
        user_id_str,
        status_bool,
    )
    .fetch_all(conn)
    .await
}

pub async fn accept_challenge(
    conn: &SqlitePool,
    challenger_id: &u64,
    challenged_id: &u64,
) -> Result<SqliteDone, sqlx::Error> {
    let challenger_id_str = challenger_id.to_string();
    let challenged_id_str = challenged_id.to_string();

    sqlx::query!(
        "UPDATE challenges SET accepted = TRUE WHERE challenger = ? AND challenged = ? AND accepted = FALSE",
        challenger_id_str,
        challenged_id_str,
    )
    .execute(conn)
    .await
}
