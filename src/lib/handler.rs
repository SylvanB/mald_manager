use super::command::{handle_or_err, MaldManager};
use crate::lib::state::*;
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
            }
            "!mald_hist" => {
                handle_or_err(MaldManager::mald_history, ctx, msg);
            }
            "!mald_hist_graph" => {
                // handle_or_err(MaldManager::mald_history_graph, ctx, msg);
                if msg.mentions.len() == 0 {
                    MaldManager::error(&ctx, &msg);
                    return;
                }

                for user in &msg.mentions {
                    let _ = MaldManager::mald_history_graph(&ctx, &msg, user);
                }
            }
            "!mald_help" => {
                MaldManager::help(&ctx, &msg);
            }
            _ => {}
        }
    }
}
