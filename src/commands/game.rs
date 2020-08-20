use crate::result::Error;
use crate::utils;
use serenity::model::channel::{Message, ReactionType};
use serenity::prelude::*;

pub async fn handle_hand(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let hand = utils::deal_hand();

    let hand_str = hand.iter().enumerate().fold(
        String::from("You hand has the following cards:\n"),
        |acc, (i, card)| {
            let emoji = match i {
                0 => ":one:",
                1 => ":two:",
                2 => ":three:",
                3 => ":four:",
                4 => ":five:",
                _ => ":100:",
            };
            acc + &format!(
                "{emoji} {color} {element} {value}\n",
                emoji = emoji,
                color = card.color,
                element = card.element,
                value = card.value
            )
        },
    );

    msg.author
        .direct_message(&ctx.http, |message| {
            message.content(hand_str);
            // message.reactions(vec![":one:", ":two:", ":three:", ":four:", ":five:"]);
            message.reactions(vec![
                ReactionType::Unicode(String::from("1️⃣")),
                ReactionType::Unicode(String::from("2️⃣")),
                ReactionType::Unicode(String::from("3️⃣")),
                ReactionType::Unicode(String::from("4️⃣")),
                ReactionType::Unicode(String::from("5️⃣")),
            ]);
            message
        })
        .await?;

    Ok(())
}
