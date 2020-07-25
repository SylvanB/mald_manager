use super::command::{handle_or_err, MaldManager};
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

        match chunks[0] {
            "!mald" => {
                handle_or_err(MaldManager::new_mald, ctx, msg);
            }
            "!demald" => {
                handle_or_err(MaldManager::demald, ctx, msg);
            },
            "!mald_hist" => {
                handle_or_err(MaldManager::mald_history, ctx, msg);
            },
            "!mald_help" => {
                MaldManager::help(&ctx, &msg);
            }
            _ => {}
        }
    }

    fn ready(&self, ctx: Context, _ready: Ready) {
        let mald_location = env::var("MALD_LOCATION").expect("Expected a token in the environment");

        let mut data = ctx.data.write();
        data.insert::<MaldData>(read_local_mald_history(mald_location).unwrap());
    }
}
