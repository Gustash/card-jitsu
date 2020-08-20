#[derive(sqlx::FromRow, Debug)]
pub struct Challenge {
    pub id: i64,
    pub user_one: String,
    pub user_two: String,
    pub challenger: String,
    pub accepted: bool,
    pub winner: Option<String>,
}

pub enum Color {
    RED,
    GREEN,
    BLUE,
}

pub enum Element {
    FIRE,
    SNOW,
    WATER,
}

pub struct Card {
    pub color: Color,
    pub element: Element,
    pub value: u8,
}
