use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::collection::Collection;

#[derive(Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::articles)]
#[diesel(belongs_to(Collection, foreign_key = collection_id))]
pub struct Article {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = crate::schema::article_search_metadata)]
pub struct ArticleSearchMetadata {
    pub id: Uuid,
    pub article_id: Uuid,
    pub summary: String,
    pub bullet_points: Vec<String>,
    pub keywords: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = crate::schema::article_chunks)]
pub struct ArticleChunk {
    pub id: Uuid,
    pub article_id: Uuid,
    pub content: String,
    pub is_title: bool,
}

// Insertable structs for creating new records

#[derive(Insertable)]
#[diesel(table_name = crate::schema::articles)]
pub struct NewArticle {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::article_search_metadata)]
pub struct NewArticleSearchMetadata {
    pub id: Uuid,
    pub article_id: Uuid,
    pub summary: String,
    pub bullet_points: Vec<String>,
    pub keywords: Vec<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::article_embeddings)]
pub struct NewArticleEmbedding {
    pub id: Uuid,
    pub metadata_id: Uuid,
    pub embedding_type: EmbeddingType,
    pub embedding: Vector,
}
