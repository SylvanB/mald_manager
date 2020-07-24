use crate::lib::state::MaldCounter;
use serenity::client::Context;
use std::{
    collections::BTreeMap,
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub(crate) enum HistoryError {
    PathDoesntExist(String),
}

pub(crate) fn read_local_mald_history(location: String) -> Option<BTreeMap<String, u64>> {
    let path = Path::new(&location);
    if !path.exists() {
        return None;
    }
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();

    let malds: BTreeMap<String, u64> =
        serde_json::from_str(&contents).unwrap_or(BTreeMap::default());

    Some(malds)
}

pub(crate) fn write_local_mald_history(ctx: &Context) -> Result<(), HistoryError> {
    let mald_location = env::var("MALD_LOCATION").expect("Expected a token in the environment");
    let data = ctx.data.read();
    let malds = data.get::<MaldCounter>().unwrap();
    let path = Path::new(&mald_location);

    if !path.exists() {
        return Err(HistoryError::PathDoesntExist(format!(
            "Location `{}` doesn't exist.",
            mald_location
        )));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .unwrap();
        
    let malds = serde_json::to_string(&malds).unwrap();

    file.write(malds.as_bytes()).unwrap();

    Ok(())
}
