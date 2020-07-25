use super::{persistance::write_local_mald_history, state};
use chrono::Utc;
use serenity::{
    client::Context,
    model::{channel::Message, prelude::User},
    utils::MessageBuilder,
};

pub(crate) struct MaldManager;
impl MaldManager {
    pub fn new_mald(ctx: &Context, msg: &Message, user: &User) {
        let date = Utc::now().format("%d/%m/%Y").to_string();
        match state::add_mald(&ctx, &date, user.id) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }

        let curr_malds = state::get_mald_count(&ctx, &date, user.id);

        let mut message = MessageBuilder::new();

        message.mention(user);

        match curr_malds {
            1 => message.push(format!(" has malded only once!")),
            _ => message.push(format!(" has malded `{}` times!", curr_malds)),
        };

        let _ = write_local_mald_history(&ctx);

        if let Err(why) = msg.channel_id.say(&ctx.http, message.build()) {
            println!("Error sending message: {:?}", why);
        }
    }

    pub fn mald_history(ctx: &Context, msg: &Message, user: &User) {
        let mut message = MessageBuilder::new();

        message.mention(user);
        message.push(" recent mald history:\n");

        let mald_history = state::get_mald_history(&ctx, user.id);

        match mald_history {
            Some(h) => {
                h.iter().for_each(|hist| {
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

    pub fn demald(ctx: &Context, msg: &Message, user: &User) {
        let date = Utc::now().format("%d/%m/%Y").to_string();
        let mut message = MessageBuilder::new();
        message.mention(user);

        match state::remove_mald(&ctx, &date, user.id) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }

        let curr_malds = state::get_mald_count(&ctx, &date, user.id);

        let _ = write_local_mald_history(&ctx);

        message.push(format!(
            " glad to see you've calmed down, todays mald level is now `{}`!",
            curr_malds
        ));

        if let Err(why) = msg.channel_id.say(&ctx.http, message.build()) {
            println!("Error sending message: {:?}", why);
        }
    }

    fn error(ctx: &Context, msg: &Message) {
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
