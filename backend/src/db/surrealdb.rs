// File: src/db/surrealdb.rs
use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error as SurrealError, RecordId, Surreal};
use tokio::sync::OnceCell;

static DB: OnceCell<Surreal<Client>> = OnceCell::const_new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub categories: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedArticle {
    pub article_id: String,
    pub summary: String,
    pub key_points: Vec<String>,
    pub keywords: Vec<String>,
    pub semantic_chunks: Vec<String>,
    pub embeddings: Vec<f32>,
    pub categories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    id: RecordId,
}

pub async fn init_surrealdb() -> Result<&'static Surreal<Client>, SurrealError> {
    DB.get_or_try_init(|| async {
        let url = env::var("SURREALDB_URL").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
        let username = env::var("SURREALDB_USER").unwrap_or_else(|_| "root".to_string());
        let password = env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".to_string());

        let db = Surreal::new::<Ws>(url).await?;
        db.signin(Root {
            username: &username,
            password: &password,
        })
        .await?;

        db.use_ns("healthtech").use_db("helpdocs").await?;
        Ok(db)
    })
    .await
}

pub async fn setup_schema() -> Result<(), SurrealError> {
    let db = init_surrealdb().await?;

    // Define table for articles
    db.query("DEFINE TABLE articles SCHEMAFULL").await?;

    // Define fields for articles
    db.query(
        r#"
        DEFINE FIELD title ON TABLE articles TYPE string;
        DEFINE FIELD content ON TABLE articles TYPE string;
        DEFINE FIELD slug ON TABLE articles TYPE string;
        DEFINE FIELD categories ON TABLE articles TYPE array;
        DEFINE FIELD created_at ON TABLE articles TYPE datetime;
        DEFINE FIELD updated_at ON TABLE articles TYPE datetime;
    "#,
    )
    .await?;

    // Define table for processed articles
    db.query("DEFINE TABLE processed_articles SCHEMAFULL")
        .await?;

    // Define fields for processed articles
    db.query(
        r#"
        DEFINE FIELD article ON TABLE processed_articles TYPE record;
        DEFINE FIELD summary ON TABLE processed_articles TYPE string;
        DEFINE FIELD key_points ON TABLE processed_articles TYPE array;
        DEFINE FIELD keywords ON TABLE processed_articles TYPE array;
        DEFINE FIELD semantic_chunks ON TABLE processed_articles TYPE array;
        DEFINE FIELD embeddings ON TABLE processed_articles TYPE array;
        DEFINE FIELD categories ON TABLE processed_articles TYPE array;
    "#,
    )
    .await?;

    Ok(())
}
