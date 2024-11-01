use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::surrealdb, schema::collections};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = collections)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub helpscout_collection_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Meta Data
    pub paragraph_description: Option<String>,
    pub bullet_points: Option<String>,
    pub keywords: Option<String>,
    pub paragraph_description_embedding: Option<Vector>,
    pub bullet_points_embedding: Option<Vector>,
    pub keywords_embedding: Option<Vector>,
}

impl Collection {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Collection>, diesel::result::Error> {
        use crate::schema::collections::table;
        table.load::<Collection>(conn)
    }

    pub fn to_surreal_collection(&self) -> Result<surrealdb::NewCollection, anyhow::Error> {
        Ok(surrealdb::NewCollection {
            id: self.id.to_string(),
            name: self.name.clone(),
            description: self.description.clone().unwrap_or_default(),
            slug: self.slug.clone(),
            helpscout_collection_id: self.helpscout_collection_id.clone(),
            paragraph_description: self.paragraph_description.clone(),
            bullet_points: self.bullet_points.clone(),
            keywords: self.keywords.clone(),
        })
    }
}
