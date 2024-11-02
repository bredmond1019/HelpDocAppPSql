use chrono::{DateTime, Utc};
use diesel::{associations::HasTable, prelude::*};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::models::collection::Collection, schema::articles};

use super::OldArticle;

#[derive(
    Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, Insertable,
)]
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

impl Article {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Article>, diesel::result::Error> {
        articles::table.load::<Article>(conn)
    }

    pub fn convert_from_old(old_article: OldArticle) -> Self {
        Self {
            id: old_article.id,
            collection_id: old_article.collection_id,
            title: old_article.title,
            slug: old_article.slug,
            content: old_article.markdown_content.unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn store(&self, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        log::info!("Storing article: ID:{:?}, Title: {:?}", self.id, self.title);

        let article: Self = diesel::insert_into(articles::table)
            .values(self)
            .get_result(conn)
            .expect("Error creating article");
        log::info!(
            "Result: Article ID: {:?}, Article Title: {:?}",
            article.id,
            article.title
        );

        Ok(article)
    }
}
