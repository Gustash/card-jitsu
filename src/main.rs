use dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv::dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("{}", token);
}
