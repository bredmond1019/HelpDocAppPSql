use chrono::{DateTime, TimeZone, Utc};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::old_collection::OldCollection;
use crate::schema::old_articles;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = crate::schema::old_articles)]
#[diesel(belongs_to(OldCollection, foreign_key = collection_id))]
pub struct OldArticle {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub title: String,
    pub slug: String,
    pub html_content: Option<String>,
    pub markdown_content: Option<String>,
    pub version: i32,
    pub last_edited_by: Option<String>,
    pub helpscout_collection_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub helpscout_article_id: Option<String>,
    // Meta Data
    pub paragraph_description: Option<String>,
    pub bullet_points: Option<Vec<Option<String>>>,
    pub keywords: Option<Vec<Option<String>>>,
    pub paragraph_description_embedding: Option<Vector>,
    pub bullet_points_embedding: Option<Vector>,
    pub keywords_embedding: Option<Vector>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct OldArticleChunk {
    pub id: Uuid,
    pub article_id: Uuid,
    pub content: String,
    pub is_title: bool,
    pub embedding_id: Option<Uuid>,
}

impl OldArticle {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<OldArticle>, diesel::result::Error> {
        old_articles::table.load::<OldArticle>(conn)
    }
}

impl HasTable for OldArticle {
    type Table = old_articles::table;

    fn table() -> Self::Table {
        old_articles::table
    }
}
