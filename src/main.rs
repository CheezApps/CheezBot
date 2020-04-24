use dotenv;
use std::env;

use serenity::prelude::Client;

mod handler;
mod commands;


fn main() {
    dotenv::dotenv().ok();

    // start discord bot
    let token = env::var("DISCORD_TOKEN").expect("Expected a discord token as environment variable!");

    let mut client = Client::new(&token, handler::Handler).expect("Error creating bot client");

    client.with_framework(commands::get_framework());

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
