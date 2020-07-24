use std::{ collections::BTreeMap, env, path::Path, fs::{OpenOptions, File}, io::{self, Read} };
use chrono::Utc;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use io::Write;

struct MaldCounter;
impl TypeMapKey for MaldCounter {
    type Value = BTreeMap<String, u64>;
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

fn get_mald_history(context: &Context) -> BTreeMap<String, u64>
{
    let data = context.data.read();
    let malds = data.get::<MaldCounter>().unwrap();
    malds.to_owned()
}

fn read_local_mald_history(location: String) -> Option<BTreeMap<String, u64>> {
    let path = Path::new(&location);
    if !path.exists() {
        return None;
    }
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();

    let malds: BTreeMap<String, u64>  = serde_json::from_str(&contents).unwrap_or(BTreeMap::default());

    Some(malds)
}

enum HistoryError {
    PathDoesntExist(String),
}

fn write_local_mald_history(location: String, ctx: &Context) -> Result<(), HistoryError> {
    let data = ctx.data.read();
    let malds = data.get::<MaldCounter>().unwrap();

    let path = Path::new(&location);
    if !path.exists() {
        return Err(HistoryError::PathDoesntExist(format!("Location `{}` doesn't exist.", location)));
    }
    let mut file = OpenOptions::new().write(true).read(true).open(path).unwrap();
    let malds = serde_json::to_string(&malds).unwrap(); 

    file.write(malds.as_bytes()).unwrap();

    Ok(())
}

struct MaldManager;
impl MaldManager {
    fn new_mald(ctx: Context, msg: Message) {
        let date = Utc::now().format("%d/%m/%Y").to_string();
        add_mald(&ctx, &date);
        
        let curr_malds = get_mald_count(&ctx, &date);

        let output_str = match curr_malds {
            1 => format!("Jon has malded only once!"),
            _ => format!("Jon has malded `{}` times!", curr_malds)
        };

        let mald_location = env::var("MALD_LOCATION")
            .expect("Expected a token in the environment");

        let _ = write_local_mald_history(mald_location, &ctx);

        if let Err(why) = msg.channel_id.say(&ctx.http, output_str) {
            println!("Error sending message: {:?}", why);
        }
    }

    fn mald_history(ctx: Context, msg: Message) {
        let output_str = get_mald_history(&ctx).iter()
            .fold("Jon's recent mald history:\n".to_string(), |mut acc, x| 
        {
            let mald_formatted = format!("`{} - {} mald(s)`\n", x.0, x.1);
            acc.push_str(mald_formatted.as_ref());
            acc
        });
        if let Err(why) = msg.channel_id.say(&ctx.http, output_str) {
            println!("Error sending message: {:?}", why);
        }
    }
}

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!mald" {
            MaldManager::new_mald(ctx, msg);
        } else if msg.content == "!mald_hist" {
            MaldManager::mald_history(ctx, msg);
        }

    }

    fn ready(&self, ctx: Context, _ready: Ready) {
        let mald_location = env::var("MALD_LOCATION")
            .expect("Expected a token in the environment");

        let mut data = ctx.data.write();
        data.insert::<MaldCounter>(read_local_mald_history(mald_location).unwrap());
    }
    
}

fn main() {
    kankyo::load(false).expect("Failed to load .env file");

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