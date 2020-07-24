use serenity::{client::Context, model::id::UserId, prelude::TypeMapKey};
use std::collections::{BTreeMap, HashMap};

pub(crate) type UserHistory = BTreeMap<String, u64>;

pub(crate) enum StateError {
    HistoryFetchError(String),
}

pub(crate) struct MaldData;
impl TypeMapKey for MaldData {
    type Value = HashMap<u64, UserHistory>;
}

pub(crate) fn add_mald<S>(context: &Context, date: S, user_id: UserId) -> Result<(), StateError>
where
    S: Into<String>,
{
    let mut data = context.data.write();

    // The MaldData should always exist? panicing in this case is expected
    let mald_data = data
        .get_mut::<MaldData>()
        .expect("Failed to get the MaldData.");

    if !mald_data.contains_key(&user_id.0) {
        let mut user_history = UserHistory::new();
        user_history.insert(date.into(), 1);
        mald_data.insert(user_id.0, user_history);
    } else {
        let user_history = mald_data
            .get_mut(&user_id.0)
            .ok_or(StateError::HistoryFetchError(
                "Failed to get reference to user's MaldHistory data.".to_string(),
            ))?;
        let entry = user_history
            .get_mut(&date.into())
            .ok_or(StateError::HistoryFetchError(
                "Failed to get reference to an entry in the user's MaldHistory.".to_string(),
            ))?;
        *entry += 1;
    }

    Ok(())
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

pub(crate) fn get_mald_history(context: &Context, user_id: UserId) -> Option<UserHistory> {
    let data = context.data.read();
    let user_data = data.get::<MaldData>()?;
    let history = user_data.get(&user_id.0)?;
    Some(history.to_owned())
}
