use super::state;
use chrono::{NaiveDate, Utc};
use serenity::{
    client::Context,
    model::{channel::Message, prelude::User},
    utils::MessageBuilder,
};
use std::{env, path::PathBuf, process::Command};

#[derive(Debug)]
pub(crate) enum CommandErr {
    FailedToSerialise,
    FailedToGenerateGraph,
    FilesLocationDoesntExist,
    FailedToGenerateFilePath,
    FailedToSortHistory,
}

pub(crate) struct MaldManager;
impl MaldManager {
    pub fn new_mald(ctx: &Context, msg: &Message, user: &User) {
        let date = Utc::now().format("%d/%m/%Y").to_string();
        match state::add_mald(&date, user.id) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }

        let curr_malds = state::get_mald_count(&date, user.id).unwrap_or(0 as u64);

        let mut message = MessageBuilder::new();

        message.mention(user);

        match curr_malds {
            1 => message.push(" has malded only once!".to_string()),
            _ => message.push(format!(" has malded `{}` times!", curr_malds)),
        };

        if let Err(why) = msg.channel_id.say(&ctx.http, message.build()) {
            println!("Error sending message: {:?}", why);
        }
    }

    pub fn mald_history(ctx: &Context, msg: &Message, user: &User) {
        let mut message = MessageBuilder::new();

        message.mention(user);
        message.push(" recent mald history:\n");

        let mald_history = state::get_mald_history(user.id);

        match mald_history {
            Some(uh) => {
                let mut malds: Vec<(String, u64)> = uh.history.into_iter().collect();
                malds.sort_by(|x, y| {
                    let x1 = MaldManager::parse_date(&x.0).unwrap();
                    let y1 = MaldManager::parse_date(&y.0).unwrap();
                    x1.cmp(&y1)
                });

                malds.iter().for_each(|hist| {
                    let mald_formatted = format!("{} - {} mald(s)", hist.0, hist.1);
                    message.push_bold_line(mald_formatted);
                });
            }
            None => {
                message.push_bold_line(format!("{} is mald free!", user.name));
            }
        }

        if let Err(why) = msg.channel_id.say(&ctx.http, message.build()) {
            println!("Error sending message: {:?}", why);
        }
    }

    pub fn mald_history_graph(ctx: &Context, msg: &Message, user: &User) -> Result<(), CommandErr> {
        let mut message = MessageBuilder::new();

        message.mention(user);
        message.push(" recent mald history:\n");

        let mald_history = state::get_mald_history(user.id)
            .and_then(|hist| {
                let mut malds: Vec<(String, u64)> = hist.history.into_iter().collect();
                malds.sort_by(|x, y| {
                    let x1 = MaldManager::parse_date(&x.0).unwrap();
                    let y1 = MaldManager::parse_date(&y.0).unwrap();
                    x1.cmp(&y1)
                });
                Some(malds)
            })
            .ok_or(CommandErr::FailedToSortHistory)?;

        let path = MaldManager::generate_graph(&mald_history, &user).unwrap();
        let path = path.as_str();

        if &mald_history.len() == &0 {
            message.push_bold_line(format!("{} is mald free!", user.name));
            let _ = msg.channel_id.say(&ctx.http, message.build());
        } else {
            if let Err(why) = msg.channel_id.send_files(&ctx.http, vec![path], |m| {
                m.content(format!("{}'s Malds over time!", user.name))
            }) {
                println!("Error sending message: {:?}", why);
            }
        }

        Ok(())
    }

    fn parse_date(date: &String) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(&date, "%d/%m/%Y").ok()
    }

    fn generate_graph(data: &Vec<(String, u64)>, user: &User) -> Result<String, CommandErr> {
        // TODO: Should probably split graph generation into it's own module
        let src = serde_json::to_string(&data).map_err(|_| CommandErr::FailedToSerialise)?;
        let files_location =
            env::var("FILES_LOCATION").map_err(|_| CommandErr::FilesLocationDoesntExist)?;

        let mut path = PathBuf::from(&files_location);
        path.push(format!("{}{}", user.id.0, ".png"));
        let path_str = path.to_str().ok_or(CommandErr::FailedToGenerateFilePath)?;

        let command = Command::new("python3")
            .args(&["./python/graph_gen.py", &src, &path_str, &user.name])
            .output()
            .map_err(|_| CommandErr::FailedToGenerateGraph)?;

        println!("{}", std::str::from_utf8(&command.stderr).unwrap());
        if !command.status.success() {
            return Err(CommandErr::FailedToGenerateGraph);
        }

        Ok(path_str.to_string())
    }

    pub fn demald(ctx: &Context, msg: &Message, user: &User) {
        let date = Utc::now().format("%d/%m/%Y").to_string();
        let mut message = MessageBuilder::new();
        message.mention(user);

        match state::remove_mald(&date, user.id) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }

        let curr_malds = state::get_mald_count(&date, user.id).unwrap_or(0 as u64);

        message.push(format!(
            " glad to see you've calmed down, todays mald level is now `{}`!",
            curr_malds
        ));

        if let Err(why) = msg.channel_id.say(&ctx.http, message.build()) {
            println!("Error sending message: {:?}", why);
        }
    }

    pub fn help(ctx: &Context, msg: &Message) {
        let message = MessageBuilder::new()
            .push_line("The command available to me are:")
            .push_line("`!mald [@User]` - Increments a user's mald level for today.")
            .push_line("`!demald [@User]` - Decrements a user's mald level for today.")
            .push_line("`!mald_hist [@User]` - View the user's mald history.")
            .push_line("`!mald_hist_graph [@User]` - View the user's mald history as a graph.")
            .push_line("`!mald_help` - View this help prompt.")
            .build();

        if let Err(why) = msg.channel_id.say(&ctx.http, message) {
            println!("Error sending message: {:?}", why);
        }
    }

    pub fn error(ctx: &Context, msg: &Message) {
        let message = MessageBuilder::new()
            .mention(&msg.author)
            .push(" oi, dickhead, that's not a real command.")
            .build();

        if let Err(why) = msg.channel_id.say(&ctx.http, message) {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub(crate) fn handle_or_err(action: fn(&Context, &Message, &User), ctx: Context, msg: Message) {
    println!("Message content: {}", msg.content);

    if msg.mentions.len() == 0 {
        MaldManager::error(&ctx, &msg);
        return;
    }

    for user in &msg.mentions {
        action(&ctx, &msg, user);
    }
}
