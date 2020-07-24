use super::command::MaldManager;
use crate::lib::{persistance::read_local_mald_history, state::*};
use serenity::{
    client::{Context, EventHandler},
    model::{channel::Message, prelude::Ready},
};
use std::env;

pub(crate) struct MaldHandler;
impl EventHandler for MaldHandler {
    fn message(&self, ctx: Context, msg: Message) {
        let chunks: Vec<&str> = msg.content.split(' ').collect();
        if chunks[0] == "!mald" {
            println!("Message content: {}", msg.content);
            for user in &msg.mentions {
                MaldManager::new_mald(&ctx, &msg, user);
            }
        } else if chunks[0] == "!mald_hist" {
            println!("Message content: {}", msg.content);
            for user in &msg.mentions {
                MaldManager::mald_history(&ctx, &msg, user);
            }
        }
    }

    fn ready(&self, ctx: Context, _ready: Ready) {
        let mald_location = env::var("MALD_LOCATION").expect("Expected a token in the environment");

        let mut data = ctx.data.write();
        data.insert::<MaldData>(read_local_mald_history(mald_location).unwrap());
    }
}
