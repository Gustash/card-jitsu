use crate::models::Card;
use crate::result::Error;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn send_message_to_channel(
    ctx: &Context,
    msg: &Message,
    content: impl std::fmt::Display,
) -> Result<Message, Error> {
    Ok(msg.channel_id.say(&ctx.http, content).await?)
}

/// Deals a hand of five cards.
///
/// # Examples
///
/// ```
/// let hand = card_jitsu::utils::deal_hand();
///
/// assert_eq!(5, hand.len());
/// ```
pub fn deal_hand() -> Vec<Card> {
    (1..6).map(|_| Card::new()).collect()
}
