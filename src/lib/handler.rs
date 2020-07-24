use super::commands::MaldManager;
use crate::lib::{persistance::read_local_mald_history, state::*};
use serenity::{
    client::{Context, EventHandler},
    model::{channel::Message, prelude::Ready},
};
use std::env;

pub(crate) struct MaldHandler;
impl EventHandler for MaldHandler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!mald" {
            MaldManager::new_mald(ctx, msg);
        } else if msg.content == "!mald_hist" {
            MaldManager::mald_history(ctx, msg);
        }
    }

    fn ready(&self, ctx: Context, _ready: Ready) {
        let mald_location = env::var("MALD_LOCATION").expect("Expected a token in the environment");

        let mut data = ctx.data.write();
        data.insert::<MaldCounter>(read_local_mald_history(mald_location).unwrap());
    }
}
