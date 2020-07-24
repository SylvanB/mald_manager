use std::{collections::HashMap, env};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct MaldCounter;
impl TypeMapKey for MaldCounter {
    type Value = HashMap<String, u64>;
}

fn add_mald<S>(context: &Context, date: S) 
    where S: Into<String> 
{
    let mut data = context.data.write();
    let counter = data.get_mut::<MaldCounter>().unwrap();
    let entry = counter.entry(date.into()).or_insert(0);
    *entry += 1;
}

fn get_mald_count<S>(context: &Context, date: S) -> u64
    where S: Into<String> 
{
    let data = context.data.read();
    let malds = data.get::<MaldCounter>().unwrap();
    let mald_count = malds.get(&date.into()).unwrap();
    *mald_count
}

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!mald" {
            add_mald(&ctx, "0");
            let curr_malds = get_mald_count(&ctx, "0");
            let output_str = match curr_malds {
                1 => format!("Jon has malded only once!"),
                _ => format!("Jon has malded `{}` times!", curr_malds)
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, output_str) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, ctx: Context, _ready: Ready) {
        let mut data = ctx.data.write();
        data.insert::<MaldCounter>(HashMap::default());
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    println!("{}", &token);

    let mut client = Client::new(&token, Handler).expect("Err creating client");
    println!("Logged in MaldManager!");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

    client.start().expect("Could not start client.");

    println!("Running MaldManager!");
}