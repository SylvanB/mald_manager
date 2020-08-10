mod lib;

use crate::lib::handler::MaldHandler;
use serenity::prelude::*;
use std::env;

fn main() {
    kankyo::load(false).expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, MaldHandler {}).expect("Err creating client");
    println!("Logged in MaldManager!");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

    client.start().expect("Could not start client.");
}
