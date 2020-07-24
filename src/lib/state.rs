use serenity::{client::Context, prelude::TypeMapKey};
use std::collections::BTreeMap;

pub(crate) struct MaldCounter;
impl TypeMapKey for MaldCounter {
    type Value = BTreeMap<String, u64>;
}

pub(crate) fn add_mald<S>(context: &Context, date: S)
where
    S: Into<String>,
{
    let mut data = context.data.write();
    let counter = data.get_mut::<MaldCounter>().unwrap();
    let entry = counter.entry(date.into()).or_insert(0);
    *entry += 1;
}

pub(crate) fn get_mald_count<S>(context: &Context, date: S) -> u64
where
    S: Into<String>,
{
    let data = context.data.read();
    let malds = data.get::<MaldCounter>().unwrap();
    let mald_count = malds.get(&date.into()).unwrap();
    *mald_count
}

pub(crate) fn get_mald_history(context: &Context) -> BTreeMap<String, u64> {
    let data = context.data.read();
    let malds = data.get::<MaldCounter>().unwrap();
    malds.to_owned()
}
