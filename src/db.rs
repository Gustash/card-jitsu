use super::models::Challenge;
use super::result::Error;
use sqlx::{sqlite::SqliteDone, SqlitePool};
use std::{cmp, env};

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
) -> Result<SqliteDone, Error> {
    let challenger_id_str = challenger_id.to_string();
    let user_one = cmp::min(challenger_id, challenged_id).to_string();
    let user_two = cmp::max(challenger_id, challenged_id).to_string();

    let result = sqlx::query!(
        "INSERT INTO challenges (user_one, user_two, challenger) VALUES (?, ?, ?)",
        user_one,
        user_two,
        challenger_id_str,
    )
    .execute(conn)
    .await?;

    Ok(result)
}

pub async fn find_challenge(
    conn: &SqlitePool,
    challenger_id: &u64,
    challenged_id: &u64,
) -> Result<Challenge, Error> {
    let user_one = cmp::min(challenger_id, challenged_id).to_string();
    let user_two = cmp::max(challenger_id, challenged_id).to_string();

    let result = sqlx::query_as!(
        Challenge,
        "SELECT * FROM challenges WHERE user_one = ? AND user_two = ?",
        user_one,
        user_two,
    )
    .fetch_one(conn)
    .await?;

    Ok(result)
}

// TODO: Look into replacing Future with Stream
pub async fn find_all_challenges(
    conn: &SqlitePool,
    user_id: &u64,
) -> Result<Vec<Challenge>, Error> {
    let user_id_str = user_id.to_string();

    let result = sqlx::query_as!(
        Challenge,
        "SELECT * FROM challenges WHERE user_one = ?1 OR user_two = ?1",
        user_id_str,
    )
    .fetch_all(conn)
    .await?;

    Ok(result)
}

pub async fn find_active_challenge(
    conn: &SqlitePool,
    user_id: &u64,
) -> Result<Option<Challenge>, Error> {
    let user_id_str = user_id.to_string();

    let result = sqlx::query_as!(
        Challenge,
        "SELECT * FROM challenges WHERE (user_one = ?1 OR user_two = ?1) AND accepted = TRUE",
        user_id_str,
    )
    .fetch_optional(conn)
    .await?;

    Ok(result)
}

pub async fn find_pending_challenges(
    conn: &SqlitePool,
    user_id: &u64,
) -> Result<Vec<Challenge>, Error> {
    let user_id_str = user_id.to_string();

    let result = sqlx::query_as!(
        Challenge,
        "SELECT * FROM challenges WHERE (user_one = ?1 OR user_two = ?1) AND accepted = FALSE",
        user_id_str,
    )
    .fetch_all(conn)
    .await?;

    Ok(result)
}

pub async fn accept_challenge(
    conn: &SqlitePool,
    challenger_id: &u64,
    challenged_id: &u64,
) -> Result<SqliteDone, Error> {
    if let Some(_) = find_active_challenge(conn, challenger_id).await? {
        return Err(Error::ExistingActiveChallenge(challenger_id.clone()));
    };
    if let Some(_) = find_active_challenge(conn, challenged_id).await? {
        return Err(Error::ExistingActiveChallenge(challenged_id.clone()));
    };

    let challenger_id_str = challenger_id.to_string();
    let user_one = cmp::min(challenger_id, challenged_id).to_string();
    let user_two = cmp::max(challenger_id, challenged_id).to_string();

    let result = sqlx::query!(
        "UPDATE challenges SET accepted = TRUE WHERE user_one = ? AND user_two = ? AND accepted = FALSE AND challenger = ?",
        user_one,
        user_two,
        challenger_id_str,
    )
    .execute(conn)
    .await?;

    Ok(result)
}
