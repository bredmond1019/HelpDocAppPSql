use anyhow::Result;
use kalosm::language::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct ArticleAnalysis {
    summary: String,
    bullet_points: Vec<String>,
    keywords: Vec<String>,
}

#[derive(Parse, Debug, Clone)]
struct ArticleStructure {
    summary: String,
    #[parse(array_length = "5..=10")]
    bullet_points: Vec<String>,
    #[parse(array_length = "5..=10")]
    keywords: Vec<String>,
}

pub struct ArticleProcessor {
    model: Arc<Mutex<Llama>>,
    embedder: Arc<Mutex<Bert>>,
    db: Arc<Surreal<Client>>,
}

impl ArticleProcessor {
    pub async fn new(db: Arc<Surreal<Client>>) -> Result<Self> {
        Ok(Self {
            model: Arc::new(Mutex::new(Llama::new_chat().await?)),
            embedder: Arc::new(Mutex::new(Bert::new().await?)),
            db,
        })
    }

    pub async fn process_article(&self, article: &crate::db::models::Article) -> Result<()> {
        // 1. Generate article analysis using structured generation
        let analysis = self.analyze_article(article).await?;

        // 2. Generate embeddings for each component
        let summary_embedding = self.embedder.lock().await.embed(&analysis.summary).await?;
        let bullets_text = analysis.bullet_points.join(" ");
        let bullets_embedding = self.embedder.lock().await.embed(&bullets_text).await?;
        let keywords_text = analysis.keywords.join(" ");
        let keywords_embedding = self.embedder.lock().await.embed(&keywords_text).await?;

        // 3. Create semantic chunks
        let chunker = SemanticChunker::new();
        let document = Document::new()
            .with_title(&article.title)
            .with_body(&article.content);
        let chunks = chunker
            .chunk(&document, &self.embedder.lock().await)
            .await?;

        // 4. Store processed article with all components in SurrealDB
        let processed_article = crate::db::models::ProcessedArticle {
            article_id: article.id.to_string(),
            summary: analysis.summary,
            key_points: analysis.bullet_points,
            keywords: analysis.keywords,
            semantic_chunks: chunks.iter().map(|c| c.text().to_string()).collect(),
            embeddings: chunks.iter().flat_map(|c| c.embedding().to_vec()).collect(),
            categories: article.categories.clone(),
        };

        // Store in SurrealDB
        self.db
            .create(("processed_articles", article.id.to_string()))
            .content(processed_article)
            .await?;

        Ok(())
    }

    async fn analyze_article(
        &self,
        article: &crate::db::models::Article,
    ) -> Result<ArticleAnalysis> {
        let parser = ArticleStructure::new_parser();

        let task = Task::builder(
            "You analyze articles and extract key information in a structured format. \
             Provide a concise summary, 5-10 key bullet points, and 5-10 relevant keywords or phrases."
        )
        .with_constraints(parser)
        .build();

        let prompt = format!(
            "Analyze the following article titled '{}': \n\n{}",
            article.title, article.content
        );

        let model = self.model.lock().await;
        let structure: ArticleStructure = task.run(&prompt, &model).await?;

        Ok(ArticleAnalysis {
            summary: structure.summary,
            bullet_points: structure.bullet_points,
            keywords: structure.keywords,
        })
    }

    pub async fn batch_process_articles(
        &self,
        articles: Vec<crate::db::models::Article>,
    ) -> Result<()> {
        for article in articles {
            if let Err(e) = self.process_article(&article).await {
                log::error!("Failed to process article {}: {}", article.id, e);
                continue;
            }
            log::info!("Successfully processed article {}", article.id);
        }
        Ok(())
    }
}

// Create a command to process articles
pub async fn process_articles_command() -> Result<()> {
    let db = crate::db::init_surrealdb().await?;
    let db = Arc::new(db);

    // Query all unprocessed articles
    let articles: Vec<crate::db::models::Article> = db
        .query("SELECT * FROM articles WHERE NOT EXISTS (SELECT * FROM processed_articles WHERE article_id = articles.id)")
        .await?
        .take(0)?;

    if articles.is_empty() {
        log::info!("No unprocessed articles found");
        return Ok(());
    }

    let processor = ArticleProcessor::new(db.clone()).await?;
    processor.batch_process_articles(articles).await?;

    Ok(())
}
