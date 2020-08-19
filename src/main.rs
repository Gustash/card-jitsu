mod commands;
mod db;
mod models;
mod result;

use commands::challenge;
use dotenv;
use rand::Rng;
use result::*;
use serenity::{
    async_trait,
    model::{
        channel::{Message, Reaction, ReactionType},
        gateway::Ready,
    },
    prelude::*,
};
use sqlx::SqlitePool;
use std::{env, fmt};

static COMMAND_PREFIX: &str = "!";

enum Color {
    RED,
    GREEN,
    BLUE,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RED => write!(f, "Red"),
            Color::GREEN => write!(f, "Green"),
            Color::BLUE => write!(f, "Blue"),
        }
    }
}

enum Element {
    FIRE,
    SNOW,
    WATER,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::FIRE => write!(f, "Fire"),
            Element::SNOW => write!(f, "Snow"),
            Element::WATER => write!(f, "Water"),
        }
    }
}

struct Card {
    color: Color,
    element: Element,
    value: u8,
}

impl Card {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        Card {
            color: match rng.gen_range(0, 3) {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::RED,
            },
            element: match rng.gen_range(0, 3) {
                0 => Element::FIRE,
                1 => Element::SNOW,
                2 => Element::WATER,
                _ => Element::FIRE,
            },
            value: rng.gen_range(1, 11),
        }
    }
}

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

#[tokio::main]
async fn main() {
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

fn deal_hand() -> Vec<Card> {
    (1..6).map(|_| Card::new()).collect()
}

async fn handle_command(
    command: &str,
    ctx: &Context,
    msg: &Message,
    conn: &SqlitePool,
) -> Result<(), Error> {
    if let Err(error) = match command {
        "hand" => handle_hand(ctx, msg).await,
        "challenge" => challenge::handle_challenge(ctx, msg, conn).await,
        "list" => challenge::handle_list_challenges(ctx, msg, conn).await,
        "accept" => challenge::handle_accept(ctx, msg, conn).await,
        _ => return Err(Error::HandleCommand(String::from(command))),
    } {
        return Err(error);
    };

    Ok(())
}

async fn handle_hand(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let hand = deal_hand();

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

    if let Err(error) = msg
        .author
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
        .await
    {
        return Err(Error::Serenity(error));
    };

    Ok(())
}
