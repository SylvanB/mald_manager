use serenity::{client::Context, prelude::TypeMapKey, model::id::UserId};
use std::collections::{HashMap, BTreeMap};

pub(crate) struct MaldData;
impl TypeMapKey for MaldData {
    type Value = HashMap<u64, BTreeMap<String, u64>>;
}

pub(crate) fn add_mald<S>(context: &Context, date: S, user_id: UserId)
where
    S: Into<String>,
{
    let mut data = context.data.write();
    let user_data = data.get_mut::<MaldData>().unwrap();
    
    if !user_data.contains_key(&user_id.0) {
        let mut history = BTreeMap::<String, u64>::new();
        history.insert(date.into(), 1);
        user_data.insert(user_id.0, history);
    } else {
        let history = user_data.get_mut(&user_id.0).unwrap();
        let entry = history.get_mut(&date.into()).unwrap();
        *entry += 1;
    }
}

pub(crate) fn get_mald_count<S>(context: &Context, date: S, user_id: UserId) -> u64
where
    S: Into<String>,
{
    let data = context.data.read();
    let user_data = data.get::<MaldData>().unwrap();
    let history = user_data.get(&user_id.0).unwrap();
    let malds = history.get(&date.into()).unwrap();
    *malds
}

pub(crate) fn get_mald_history(context: &Context, user_id: UserId) -> BTreeMap<String, u64> {
    let data = context.data.read();
    let user_data = data.get::<MaldData>().unwrap();
    let history = user_data.get(&user_id.0).unwrap();
    history.to_owned()
}
