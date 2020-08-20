use crate::db;
use crate::models::Challenge;
use crate::result::*;
use crate::utils;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use sqlx::{Done, SqlitePool};

pub async fn handle_challenge(
    ctx: &Context,
    msg: &Message,
    conn: &SqlitePool,
) -> Result<(), Error> {
    {
        let mention_error = match msg.mentions.len() {
            len if len == 0 => Some("You need to challenge at least one user"),
            len if len == 1 => {
                if msg.mentions[0] == msg.author {
                    Some("You cannot challenge yourself, silly!")
                } else {
                    None
                }
            }
            len if len > 1 => Some("You can only challenge one user at a time"),
            _ => None,
        };

        if let Some(error_msg) = mention_error {
            utils::send_message_to_channel(ctx, msg, error_msg).await?;
            return Err(Error::HandleCommand(error_msg.to_string()));
        }
    }

    let mention = &msg.mentions[0];

    match db::create_challenge(conn, msg.author.id.as_u64(), mention.id.as_u64()).await {
        Ok(_) => Ok(handle_challenge_success(ctx, msg).await?),
        Err(error) => Err(handle_challenge_failure(ctx, msg, conn, error).await),
    }
}

async fn handle_challenge_success(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let mention = &msg.mentions[0];

    let content = MessageBuilder::new()
        .mention(&msg.author)
        .push(" has challenged ")
        .mention(mention)
        .build();

    utils::send_message_to_channel(ctx, msg, content).await?;

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
            .push("challenge with ")
            .mention(mention)
            .build(),
        Err(_error) => {
            error = Some(_error);

            MessageBuilder::new()
                .push("There was a problem challenging ")
                .mention(mention)
                .build()
        }
    };

    if let Err(_error) = utils::send_message_to_channel(ctx, msg, content).await {
        error = Some(_error);
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
    let pending_challenges = db::find_pending_challenges(conn, msg.author.id.as_u64()).await?;
    let ongoing_challenge = db::find_active_challenge(conn, msg.author.id.as_u64()).await?;

    let mut message_builder = MessageBuilder::new();
    if pending_challenges.is_empty() && ongoing_challenge.is_none() {
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
        if let Some(challenge) = ongoing_challenge {
            build_challenge_list_message(
                ctx,
                &mut message_builder,
                vec![challenge],
                msg.author.id.as_u64(),
                true,
            )
            .await;
        }
    }

    utils::send_message_to_channel(ctx, msg, message_builder.build()).await?;

    Ok(())
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
                "Ongoing Challenge:"
            } else {
                "Pending Challenges:"
            });
        }
        let other_user_id = if challenge.user_one == author_id.to_string() {
            &challenge.user_two
        } else {
            &challenge.user_one
        };
        let other_user_id_u64 = challenge.user_two.parse::<u64>();

        if let Err(_) = other_user_id_u64 {
            println!("{} is not a valid user id", other_user_id);
            continue;
        }

        match ctx.http.get_user(other_user_id_u64.unwrap()).await {
            Ok(other_user) => {
                message_builder
                    .push("You have ")
                    .push(if ongoing { "an ongoing " } else { "a pending " })
                    .push("challenge with ")
                    .mention(&other_user)
                    .push_line("");
            }
            Err(error) => println!("Serenity error: {}", error),
        };
    }
}

pub async fn handle_accept(ctx: &Context, msg: &Message, conn: &SqlitePool) -> Result<(), Error> {
    {
        let mentions_err_message = match msg.mentions.len() {
            0 => Some("You need to tell me whose challenge to accept"),
            len if len > 1 => Some("You can only accept one person's challenge at a time"),
            _ => None,
        };
        if let Some(err_message) = mentions_err_message {
            utils::send_message_to_channel(ctx, msg, err_message).await?;
            return Err(Error::HandleCommand(
                "You need to tell me whose challenge to accept".to_string(),
            ));
        }
    }

    let author_id = msg.author.id.as_u64();
    let mention = &msg.mentions[0];
    let mention_id = mention.id.as_u64();

    match db::accept_challenge(conn, mention_id, author_id).await {
        Ok(done) => {
            if done.rows_affected() < 1 {
                let err_message = MessageBuilder::new()
                    .push("There's no challenge to accept from ")
                    .mention(mention)
                    .build();
                utils::send_message_to_channel(ctx, msg, &err_message).await?;
                return Err(Error::HandleCommand(err_message.to_string()));
            }
        }
        Err(error) => {
            let err_message = "Chucks. I couldn't accept this challenge... :crying_cat_face:";
            utils::send_message_to_channel(ctx, msg, err_message).await?;
            return Err(error);
        }
    }

    let content = MessageBuilder::new()
        .mention(&msg.author)
        .push(" has accepted ")
        .mention(mention)
        .push("'s challenge!")
        .build();

    utils::send_message_to_channel(ctx, msg, content).await?;

    Ok(())
}
