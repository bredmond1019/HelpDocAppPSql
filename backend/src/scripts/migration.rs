use log::info;

use crate::db::{self, Article, OldArticle, OldCollection};

pub fn migrate() -> Result<(), anyhow::Error> {
    // migrate_collections()?;
    migrate_articles()?;
    Ok(())
}

pub fn migrate_collections() -> Result<(), anyhow::Error> {
    let pool = db::init_pool();
    let mut conn = pool.get()?;

    let old_collections = OldCollection::load_all(&mut conn)?;

    info!("Found {} old collections", old_collections.len());
    info!("Migrating collections...");

    for old_collection in old_collections {
        info!(
            "Migrating collection: ID:{:?}, Title: {:?}",
            old_collection.id, old_collection.name
        );

        let new_collection = old_collection.convert_to_new();
        new_collection.store(&mut conn)?;
        info!("Collection stored successfully");
    }

    Ok(())
}

pub fn migrate_articles() -> Result<(), anyhow::Error> {
    let pool = db::init_pool();
    let mut conn = pool.get()?;

    let old_articles = OldArticle::load_all(&mut conn)?;

    info!("Found {} old articles", old_articles.len());
    info!("Migrating articles...");

    for old_article in old_articles {
        info!(
            "Migrating article: ID:{:?}, Title: {:?}",
            old_article.id, old_article.title
        );

        match Article::find(old_article.id, &mut conn) {
            Ok(_) => info!("Article already exists"),
            Err(_) => {
                if let Some(new_article) = old_article.convert_to_new() {
                    info!(
                        "Storing article: ID:{:?}, Title: {:?}",
                        new_article.id, new_article.title
                    );
                    new_article.store(&mut conn)?;
                    info!("Article stored successfully");
                } else {
                    info!("Article had no markdown content");
                }
            }
        }
    }

    info!("Migrating completed successfully");

    Ok(())
}
