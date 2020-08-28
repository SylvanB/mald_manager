use rustbreak::{deser::Ron, FileDatabase};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, path::Path};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UserHistory {
    pub id: u64,
    pub history: HashMap<String, u64>,
}
impl UserHistory {
    pub fn new(id: u64) -> UserHistory {
        UserHistory {
            id,
            history: HashMap::<String, u64>::new(),
        }
    }
}

trait Repository {
    fn get(id: String) -> Option<UserHistory>;

    fn upsert(id: String) -> Option<UserHistory>;

    fn delete(id: String) -> Option<UserHistory>;
}

#[derive(Debug, Error)]
pub(crate) enum DbError {
    #[error("failed to inititalise the database")]
    FailedToInitialise { source: rustbreak::RustbreakError },

    #[error("provided database location doesn't exist")]
    DbLocationDoesntExist,

    #[error("couldnt fetch value")]
    FailedToFetchValue,

    #[error("couldnt update current value")]
    FailedToUpdateValue,

    #[error("couldnt find DB location on ENV")]
    FailedToLocateDbEnvLocation,

    #[error("failed to save DB state")]
    FailedToSaveDatabaseChanges,
}

pub(crate) struct DatabaseStorage {
    connection: FileDatabase<HashMap<u64, UserHistory>, Ron>,
}

impl DatabaseStorage {
    pub fn new() -> Result<DatabaseStorage, DbError> {
        let mald_location =
            env::var("MALD_LOCATION").map_err(|err| DbError::FailedToLocateDbEnvLocation)?;
        let path = Path::new(&mald_location);

        let conn = FileDatabase::load_from_path_or_default(path)
            .map_err(|source| DbError::FailedToInitialise { source })?;

        Ok(DatabaseStorage { connection: conn })
    }

    pub fn get(&self, id: u64) -> Result<UserHistory, DbError> {
        let result = self.connection.read(|d| d.get(&id).cloned());

        // oh god this is gross pls look away
        let value = match result {
            Ok(r) => r,
            Err(_) => return Err(DbError::FailedToFetchValue),
        };

        Ok(match value {
            Some(v) => v,
            None => return Err(DbError::FailedToFetchValue),
        })
    }

    pub fn upsert(&self, id: u64, user_history: UserHistory) -> Result<(), DbError> {
        let result = self.connection.write(|db| db.insert(id, user_history));

        match result {
            Ok(_) => match self.connection.save() {
                Ok(_) => Ok(()),
                Err(_) => Err(DbError::FailedToSaveDatabaseChanges),
            },
            Err(_) => Err(DbError::FailedToUpdateValue),
        }
    }

    pub fn delete(&self, id: u64) -> Result<(), DbError> {
        let result = self.connection.write(|db| db.remove(&id));

        match result {
            Ok(_) => match self.connection.save() {
                Ok(_) => Ok(()),
                Err(_) => Err(DbError::FailedToSaveDatabaseChanges),
            },
            Err(_) => Err(DbError::FailedToUpdateValue),
        }
    }
}
