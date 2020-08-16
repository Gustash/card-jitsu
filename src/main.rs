use dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, fmt};

enum Color {
    RED,
    GREEN,
    BLUE,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Color::RED => write!(f, "Red"),
            Color::GREEN => write!(f, "Green"),
            Color::BLUE => write!(f, "Blue"),
        };
    }
}

enum Element {
    FIRE,
    SNOW,
    WATER,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Element::FIRE => write!(f, "Fire"),
            Element::SNOW => write!(f, "Snow"),
            Element::WATER => write!(f, "Water"),
        };
    }
}

struct Card {
    color: Color,
    element: Element,
    value: i8,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!hand" {
            let hand = vec![
                Card {
                    color: Color::RED,
                    element: Element::FIRE,
                    value: 1,
                },
                Card {
                    color: Color::GREEN,
                    element: Element::WATER,
                    value: 10,
                },
            ];

            let hand_str = hand.iter().fold(String::new(), |acc, card| {
                acc + &format!(
                    "Your hand has a {color} {element} card with value {value}\n",
                    color = card.color,
                    element = card.element,
                    value = card.value
                )
            });

            if let Err(why) = msg.channel_id.say(&ctx.http, hand_str).await {
                println!("Error sending message: {:?}", why);
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
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv::dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
