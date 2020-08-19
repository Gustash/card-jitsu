#[derive(sqlx::FromRow)]
pub struct Challenge {
    pub id: i64,
    pub challenger: String,
    pub challenged: String,
    pub accepted: bool,
    pub winner: Option<String>,
}

pub enum ChallengeStatus {
    Pending,
    Ongoing,
}
