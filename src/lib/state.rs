use super::db::{DatabaseStorage, UserHistory};
use serenity::{client::Context, model::id::UserId, prelude::TypeMapKey};
use std::{
    collections::{BTreeMap, HashMap},
    env,
};
use thiserror::Error;

// pub(crate) type UserHistory = BTreeMap<String, u64>;

#[derive(Debug, Error)]
pub(crate) enum StateError {
    #[error("failed to initialise DB")]
    FailedToInitialiseDb,

    #[error("Failed to get user history")]
    FailedToFetchUserHistory,

    #[error("Failed tog update user history")]
    FailedToUpdateValue,
}

pub(crate) fn add_mald<S>(date: S, user_id: UserId) -> Result<(), StateError>
where
    S: Into<String>,
{
    let db = DatabaseStorage::new().map_err(|_| StateError::FailedToInitialiseDb)?;
    let mut user_history = match db.get(user_id.0)
        .map_err(|_| StateError::FailedToFetchUserHistory) {
        Ok(uh) => uh,
        Err(_) =>  {
            match db.upsert(user_id.0, UserHistory::new(user_id.0))
            .map_err(|_| StateError::FailedToFetchUserHistory) {
                Ok(_) => {
                    db.get(user_id.0).unwrap()
                },
                Err(e) => return Err(e)
            }
        }
    };

    let new_value: u64;
    let date: String = date.into();
    if let Some(curr_value) = user_history.history.get(&date) {
        new_value = curr_value + 1;
    } else {
        new_value = 1;
    }

    user_history.history.insert(date, new_value);

    match db.upsert(user_id.0, user_history) {
        Ok(_) => {},
        Err(_) => return Err(StateError::FailedToUpdateValue)
    }

    Ok(())
}

pub(crate) fn remove_mald<S>(date: S, user_id: UserId) -> Result<(), StateError>
where
    S: Into<String>,
{
    let db = DatabaseStorage::new().map_err(|_| StateError::FailedToInitialiseDb)?;
    let mut user_history = db
        .get(user_id.0)
        .map_err(|_| StateError::FailedToFetchUserHistory)?;

    let new_value: u64;
    let date: String = date.into();
    if let Some(curr_value) = user_history.history.get(&date) {
        new_value = curr_value - 1;
    } else {
        new_value = 0;
    }

    user_history.history.insert(date.into(), new_value);

    match db.upsert(user_id.0, user_history) {
        Ok(_) => {},
        Err(_) => return Err(StateError::FailedToUpdateValue)
    }

    Ok(())
}

pub(crate) fn get_mald_count<S>(date: S, user_id: UserId) -> Result<u64, StateError>
where
    S: Into<String>,
{
    let db = DatabaseStorage::new().map_err(|_| StateError::FailedToInitialiseDb)?;
    let user_history = db
        .get(user_id.0)
        .map_err(|_| StateError::FailedToFetchUserHistory)?;

    let curr_value = user_history.history.get(&date.into());
    let result: u64;
    if let Some(curr_value) = curr_value {
        result = *curr_value;
    } else {
        result = 0;
    }

    Ok(result)
}

pub(crate) fn get_mald_history(user_id: UserId) -> Option<UserHistory> {
    let db = DatabaseStorage::new().ok()?;
    let user_history = db.get(user_id.0).ok()?;

    Some(user_history)
}
