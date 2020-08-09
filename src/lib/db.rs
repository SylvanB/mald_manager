use rustbreak::{deser::Ron, FileDatabase};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone)]
struct UserHistory {
    id: u64,
    history: HashMap<String, u64>,
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
}

struct DatebaseStorage {
    connection: FileDatabase<HashMap<u64, UserHistory>, Ron>,
}

impl DatebaseStorage {
    fn new(location: String) -> Result<DatebaseStorage, DbError> {
        let path = Path::new(&location);

        if !path.exists() {
            return Err(DbError::DbLocationDoesntExist);
        }

        let conn = FileDatabase::load_from_path(path)
            .map_err(|source| DbError::FailedToInitialise { source })?;

        Ok(DatebaseStorage { connection: conn })
    }

    fn get(self, id: u64) -> Result<UserHistory, DbError> {
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

    fn upsert(self, id: u64, user_history: UserHistory) -> Result<(), DbError> {
        let result = self.connection.write(|db| db.insert(id, user_history));

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DbError::FailedToUpdateValue),
        }
    }

    fn delete(self, id: u64) -> Result<(), DbError> {
        let result = self.connection.write(|db| db.remove(&id));

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DbError::FailedToUpdateValue),
        }
    }
}
