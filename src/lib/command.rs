use super::{
    persistance::write_local_mald_history,
    state::{add_mald, get_mald_count, get_mald_history},
};
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
        add_mald(&ctx, &date);

        let curr_malds = get_mald_count(&ctx, &date);


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

    pub fn mald_history(ctx: Context, msg: Message) {
        let output_str = get_mald_history(&ctx).iter().fold(
            "Jon's recent mald history:\n".to_string(),
            |mut acc, x| {
                let mald_formatted = format!("`{} - {} mald(s)`\n", x.0, x.1);
                acc.push_str(mald_formatted.as_ref());
                acc
            },
        );
        if let Err(why) = msg.channel_id.say(&ctx.http, output_str) {
            println!("Error sending message: {:?}", why);
        }
    }
}
