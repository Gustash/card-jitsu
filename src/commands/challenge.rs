use crate::db;
use crate::models::{Challenge, ChallengeStatus};
use crate::result::*;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use sqlx::SqlitePool;

pub async fn handle_challenge(
    ctx: &Context,
    msg: &Message,
    conn: &SqlitePool,
) -> Result<(), Error> {
    let mention_error = match msg.mentions.len() {
        len if len == 0 => Some("You need to challenge at least one user"),
        len if len > 1 => Some("You can only challenge one user at a time"),
        _ => None,
    };

    if let Some(error_msg) = mention_error {
        if let Err(error) = msg.channel_id.say(&ctx.http, error_msg).await {
            return Err(Error::Serenity(error));
        };
        return Err(Error::HandleCommand(error_msg.to_string()));
    }

    let mention = &msg.mentions[0];

    match db::create_challenge(conn, msg.author.id.as_u64(), mention.id.as_u64()).await {
        Ok(_) => match handle_challenge_success(ctx, msg).await {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::Serenity(error)),
        },
        Err(error) => {
            println!("DB error: {}", error);
            Err(handle_challenge_failure(ctx, msg, conn, Error::SQLX(error)).await)
        }
    }
}

async fn handle_challenge_success(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let mention = &msg.mentions[0];

    if let Err(error) = msg
        .channel_id
        .say(
            &ctx.http,
            MessageBuilder::new()
                .mention(&msg.author)
                .push(" has challenged ")
                .mention(mention)
                .build(),
        )
        .await
    {
        return Err(error);
    };

    Ok(())
}

async fn handle_challenge_failure(
    ctx: &Context,
    msg: &Message,
    conn: &SqlitePool,
    original_error: Error,
) -> Error {
    let mut error: Option<Error> = None;
    let mention = &msg.mentions[0];
    let result = db::find_challenge(conn, msg.author.id.as_u64(), mention.id.as_u64()).await;

    let content = match result {
        Ok(challenge) => MessageBuilder::new()
            .push("You already have ")
            .push(if challenge.accepted {
                "an ongoing "
            } else {
                "a pending "
            })
            .push("with ")
            .mention(mention)
            .build(),
        Err(_error) => {
            error = Some(Error::SQLX(_error));

            MessageBuilder::new()
                .push("There was a problem challenging ")
                .mention(mention)
                .build()
        }
    };

    if let Err(_error) = msg.channel_id.say(&ctx.http, content).await {
        error = Some(Error::Serenity(_error));
    };

    match error {
        Some(error) => error,
        None => original_error,
    }
}

pub async fn handle_list_challenges(
    ctx: &Context,
    msg: &Message,
    conn: &SqlitePool,
) -> Result<(), Error> {
    let pending_challenges =
        match db::find_challenges(conn, msg.author.id.as_u64(), ChallengeStatus::Pending).await {
            Ok(challenges) => challenges,
            Err(error) => return Err(Error::SQLX(error)),
        };
    let ongoing_challenges =
        match db::find_challenges(conn, msg.author.id.as_u64(), ChallengeStatus::Ongoing).await {
            Ok(challenges) => challenges,
            Err(error) => return Err(Error::SQLX(error)),
        };

    let mut message_builder = MessageBuilder::new();
    if pending_challenges.is_empty() && ongoing_challenges.is_empty() {
        message_builder.push("You don't have any pending or active challenges :cold_sweat:");
    } else {
        build_challenge_list_message(
            ctx,
            &mut message_builder,
            pending_challenges,
            msg.author.id.as_u64(),
            false,
        )
        .await;
        build_challenge_list_message(
            ctx,
            &mut message_builder,
            ongoing_challenges,
            msg.author.id.as_u64(),
            true,
        )
        .await;
    }

    if let Err(error) = msg.channel_id.say(&ctx.http, message_builder.build()).await {
        Err(Error::Serenity(error))
    } else {
        Ok(())
    }
}

async fn build_challenge_list_message(
    ctx: &Context,
    message_builder: &mut MessageBuilder,
    challenges: Vec<Challenge>,
    author_id: &u64,
    ongoing: bool,
) {
    for (i, challenge) in challenges.iter().enumerate() {
        if i == 0 {
            message_builder.push_bold_line_safe(if ongoing {
                "Ongoing Challenges:"
            } else {
                "Pending Challenges:"
            });
        }
        let is_challenger = challenge.challenger == author_id.to_string();
        let other_user_id = if is_challenger {
            challenge.challenged.parse::<u64>()
        } else {
            challenge.challenger.parse::<u64>()
        };

        if let Err(_) = other_user_id {
            println!(
                "{} is not a valid user id",
                if is_challenger {
                    &challenge.challenged
                } else {
                    &challenge.challenger
                }
            );
            continue;
        }

        match ctx.http.get_user(other_user_id.unwrap()).await {
            Ok(other_user) => {
                message_builder
                    .push("You have a ")
                    .push(if ongoing { "ongoing " } else { "pending " })
                    .push("challenge with ")
                    .mention(&other_user)
                    .push_line("");
            }
            Err(error) => println!("Serenity error: {}", error),
        };
    }
}

pub async fn handle_accept(ctx: &Context, msg: &Message, conn: &SqlitePool) -> Result<(), Error> {
    match msg.mentions.len() {
        0 => {
            return Err(Error::HandleCommand(
                "You need to tell me whose challenge to accept".to_string(),
            ))
        }
        len if len > 1 => {
            return Err(Error::HandleCommand(
                "You can only accept one person's challenge at a time".to_string(),
            ))
        }
        _ => (),
    };

    let author_id = msg.author.id.as_u64();
    let mention = &msg.mentions[0];
    let mention_id = mention.id.as_u64();

    match db::accept_challenge(conn, mention_id, author_id).await {
        Ok(done) => {
            // if done.changes < 1 {
            //     if let Err(error) = msg
            //         .channel_id
            //         .say(&ctx.http, "There was no pending challenge")
            //         .await
            //     {
            //         return Err(Error::Serenity(error));
            //     }
            // }
        }
        Err(error) => {
            if let Err(error) = msg
                .channel_id
                .say(
                    &ctx.http,
                    "Chucks. I couldn't accept this challenge... :crying_cat_face:",
                )
                .await
            {
                return Err(Error::Serenity(error));
            }
            return Err(Error::SQLX(error));
        }
    }

    let content = MessageBuilder::new()
        .mention(&msg.author)
        .push(" has accepted ")
        .mention(mention)
        .push("'s challenge!")
        .build();

    if let Err(error) = msg.channel_id.say(&ctx.http, content).await {
        return Err(Error::Serenity(error));
    }

    Ok(())
}
