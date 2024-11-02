use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::collections;

use super::OldCollection;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = collections)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Collection {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Collection>, diesel::result::Error> {
        use crate::schema::collections::table;
        table.load::<Collection>(conn)
    }

    pub fn convert_from_old(old_collection: OldCollection) -> Self {
        Self {
            id: old_collection.id,
            name: old_collection.name,
            description: old_collection.description,
            slug: old_collection.slug,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn store(&self, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        log::info!(
            "Storing collection: ID:{:?}, Title: {:?}",
            self.id,
            self.name
        );

        let collection: Self = diesel::insert_into(collections::table)
            .values(self)
            .get_result(conn)
            .expect("Error creating collection");
        log::info!(
            "Result: Collection ID: {:?}, Collection Title: {:?}",
            collection.id,
            collection.name
        );

        Ok(collection)
    }
}
