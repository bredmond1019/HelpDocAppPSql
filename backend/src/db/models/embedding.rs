use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::EmbeddingType"]
pub enum EmbeddingType {
    Summary,
    BulletPoints,
    Keywords,
    Chunk,
}

#[derive(Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::embeddings)]
pub struct Embedding {
    pub id: Uuid,
    pub source_id: Uuid,
    pub embedding_type: EmbeddingType,
    pub embedding: Vector,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::embeddings)]
pub struct NewEmbedding {
    pub id: Uuid,
    pub source_id: Uuid,
    pub embedding_type: EmbeddingType,
    pub embedding: Vector,
}
