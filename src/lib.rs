mod commands;
mod db;
mod models;
mod prelude;
mod result;
pub mod utils;

use commands::{challenge, game};
use dotenv;
use result::*;
use serenity::{
    async_trait,
    model::{
        channel::{Message, Reaction},
        gateway::Ready,
    },
    prelude::*,
};
use sqlx::SqlitePool;
use std::env;

static COMMAND_PREFIX: &str = "!";

struct Handler {
    conn: SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with(COMMAND_PREFIX) {
            return;
        }

        if let Err(error) =
            handle_command(extract_command(&msg.content), &ctx, &msg, &self.conn).await
        {
            match error {
                Error::Serenity(error) => println!("Error handling command: {}", error),
                Error::SQLX(error) => println!("DB error: {}", error),
                Error::HandleCommand(command) => println!("Could not handle command: {}", command),
                Error::ExistingActiveChallenge(user_id) => {
                    let user = ctx.http.get_user(user_id).await;

                    match user {
                        Ok(user) => {
                            println!("Error: {} already has an active challenge", user.name)
                        }
                        Err(error) => println!("Discord error: {}", error),
                    }
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // This handler is called whenever there's a new reaction to a message
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Ok(_message) = reaction.message(&ctx.http).await {};
    }
}

pub async fn start_client() {
    // Load .env file
    dotenv::dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler {
            conn: db::connect_to_db().await,
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

fn extract_command(content: &str) -> &str {
    match content[1..].split_whitespace().next() {
        Some(command) => command,
        None => {
            println!("No command found");
            ""
        }
    }
}

async fn handle_command(
    command: &str,
    ctx: &Context,
    msg: &Message,
    conn: &SqlitePool,
) -> Result<(), Error> {
    match command {
        "hand" => game::handle_hand(ctx, msg).await,
        "challenge" => challenge::handle_challenge(ctx, msg, conn).await,
        "list" => challenge::handle_list_challenges(ctx, msg, conn).await,
        "accept" => challenge::handle_accept(ctx, msg, conn).await,
        _ => Err(Error::HandleCommand(String::from(command))),
    }
}
