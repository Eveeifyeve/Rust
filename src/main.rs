mod handler;
mod commands;
mod events;
use serenity::prelude::*;



use handler::Handler;

#[tokio::main]
async fn main() {
    let token = "your bot token here";

    let application_id: u64 = "your application id here".parse().expect("application id is not a valid id");

    let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS )
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
