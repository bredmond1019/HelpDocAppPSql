// File: src/scripts/migrate.rs
use crate::db::{
    self,
    models::Article,
    surrealdb::{self, ProcessedArticle},
};
use anyhow::{Context, Result};
use diesel::prelude::*;
use log::{error, info};

pub async fn migrate_data() -> Result<()> {
    info!("Starting migration from PostgreSQL to SurrealDB");

    // Initialize both databases
    let mut pg_conn = db::init_pool().get()?;
    let surreal_db = surrealdb::init_surrealdb().await?;

    // Setup SurrealDB schema
    surrealdb::setup_schema().await?;

    // Migrate articles
    info!("Migrating articles...");
    let articles = Article::load_all(&mut pg_conn).context("Failed to load articles")?;

    for article in articles {
        let surreal_article = surrealdb::Article {
            id: article.id.to_string(),
            title: article.title,
            content: article.markdown_content.unwrap_or(String::new()),
            slug: article.slug,
            categories: vec![],
            created_at: article.created_at.to_rfc3339(),
            updated_at: article.updated_at.to_rfc3339(),
        };

        // let created = surreal_db
        //     .create(("articles", &surreal_article.id))
        //     .content(surreal_article)
        //     .await?;

        // match surreal_db
        //     .create::<surrealdb::Article>(("articles", &surreal_article.id))
        //     .content(surreal_article)
        //     .await
        // {
        //     Ok(_) => info!("Migrated article {}", article.id),
        //     Err(e) => error!("Failed to migrate article {}: {}", article.id, e),
        // }
    }

    // Migrate vector embeddings and processed articles
    // info!("Migrating processed articles and embeddings...");
    // let processed_articles = sql_query(
    //     "SELECT a.id as article_id, pa.* FROM processed_articles pa
    //      JOIN articles a ON a.id = pa.article_id",
    // )
    // .load::<ProcessedArticle>(&mut pg_conn)?;

    // for proc_article in processed_articles {
    //     let surreal_proc_article = surrealdb::ProcessedArticle {
    //         article_id: proc_article.article_id.to_string(),
    //         summary: proc_article.summary,
    //         key_points: proc_article.key_points,
    //         keywords: proc_article.keywords,
    //         semantic_chunks: proc_article.semantic_chunks,
    //         embeddings: proc_article.embeddings,
    //         categories: proc_article.categories,
    //     };

    //     match surreal_db
    //         .create(("processed_articles", &surreal_proc_article.article_id))
    //         .content(surreal_proc_article)
    //         .await
    //     {
    //         Ok(_) => info!("Migrated processed article {}", proc_article.article_id),
    //         Err(e) => error!(
    //             "Failed to migrate processed article {}: {}",
    //             proc_article.article_id, e
    //         ),
    //     }
    // }

    info!("Migration completed successfully");
    Ok(())
}
