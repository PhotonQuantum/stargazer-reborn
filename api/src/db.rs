//! Database access.

use std::collections::HashMap;

use color_eyre::Result;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::change_stream::event::OperationType;
use mongodb::options::{ChangeStreamOptions, FullDocumentType};
use mongodb::{bson, Client, Collection};
use tracing::{error, info};

use sg_core::models::{InDB, Task};

use crate::{get_config, Config};

/// Database instance.
pub struct DB {
    collection: Collection<InDB<i32>>,
}

impl DB {
    /// Create a new DB instance.
    ///
    /// # Errors
    /// Returns an error if the database connection fails.
    pub async fn new() -> Result<DB> {
        let config = get_config();
        let client = Client::with_uri_str(&config.mongo_uri).await?;
        let db = client.database(&config.mongo_db);
        let collection = db.collection(&config.mongo_collection);

        Ok(Self { collection })
    }

    pub async fn new_session() -> Result<()> {
        Ok(())
    }

    pub async fn lookup_session(session_id: String) -> Result<()> {
        Ok(())
    }
}
