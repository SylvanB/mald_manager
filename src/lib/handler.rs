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
        if msg.content.contains("!mald") {
            for user in &msg.mentions {
                MaldManager::new_mald(&ctx, &msg, user);
            }
        } else if msg.content.contains("!mald_hist") {
            MaldManager::mald_history(ctx, msg);
        }
    }

    fn ready(&self, ctx: Context, _ready: Ready) {
        let mald_location = env::var("MALD_LOCATION").expect("Expected a token in the environment");

        let mut data = ctx.data.write();
        data.insert::<MaldCounter>(read_local_mald_history(mald_location).unwrap());
    }
}
