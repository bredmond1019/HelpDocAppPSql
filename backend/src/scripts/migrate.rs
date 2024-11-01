// File: src/scripts/migrate.rs
use crate::db::{
    self,
    models::Article,
    surrealdb::{self, ProcessedArticle},
    Collection,
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

    // Migrate collections first
    info!("Migrating collections...");
    let collections = Collection::load_all(&mut pg_conn)?;

    for collection in collections {
        let surreal_collection = collection.to_surreal_collection()?;

        let created: Option<surrealdb::Record> = surreal_db
            .create(("collections", &surreal_collection.id))
            .content(surreal_collection)
            .await?;

        info!(
            "Migrated collection {}, created: {:?}",
            collection.id, created
        );
    }

    // Migrate articles
    // info!("Migrating articles...");
    // let articles = Article::load_all(&mut pg_conn)?;

    // for article in articles {
    //     let surreal_article = article.to_surreal_article()?;

    //     let created: Option<surrealdb::Record> = surreal_db
    //         .create(("articles", &surreal_article.id))
    //         .content(surreal_article)
    //         .await?;

    //     info!("Migrated article {}, created: {:?}", article.id, created);
    // }

    info!("Migration completed successfully");
    Ok(())
}
