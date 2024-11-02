use log::info;

use crate::db::{self, OldArticle};

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

    info!("Migrating completed successfully");

    Ok(())
}
