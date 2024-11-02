-- This file should undo anything in `up.sql`
ALTER INDEX idx_old_collections_slug RENAME TO idx_collections_slug;
ALTER INDEX idx_old_articles_slug RENAME TO idx_articles_slug;
ALTER INDEX idx_old_articles_collection_id RENAME TO idx_articles_collection_id;
-- ALTER INDEX idx_old_article_chunks_article_id RENAME TO idx_article_chunks_article_id;
ALTER INDEX idx_old_embeddings_article_id RENAME TO idx_embeddings_article_id;

-- Revert Primary keys
ALTER INDEX idx_old_article_chunks_pkey RENAME TO article_chunks_pkey;
ALTER INDEX idx_old_articles_pkey RENAME TO articles_pkey;
ALTER INDEX idx_old_collections_pkey RENAME TO collections_pkey;
ALTER INDEX idx_old_embeddings_pkey RENAME TO embeddings_pkey;

-- Revert Collections indexes
ALTER INDEX idx_old_collections_slug_key RENAME TO collections_slug_key;
ALTER INDEX idx_old_collections_helpscout_id RENAME TO idx_collections_helpscout_id;
ALTER INDEX idx_old_collections_bullet_points_embedding RENAME TO idx_collections_bullet_points_embedding;
ALTER INDEX idx_old_collections_keywords_embedding RENAME TO idx_collections_keywords_embedding;
ALTER INDEX idx_old_collections_paragraph_description_embedding RENAME TO idx_collections_paragraph_description_embedding;

-- Revert Articles indexes
ALTER INDEX idx_old_articles_bullet_points_embedding RENAME TO idx_articles_bullet_points_embedding;
ALTER INDEX idx_old_articles_content RENAME TO idx_articles_content;
ALTER INDEX idx_old_articles_helpscout_id RENAME TO idx_articles_helpscout_id;
ALTER INDEX idx_old_articles_keywords_embedding RENAME TO idx_articles_keywords_embedding;
ALTER INDEX idx_old_articles_paragraph_description_embedding RENAME TO idx_articles_paragraph_description_embedding;
ALTER INDEX idx_old_articles_title RENAME TO idx_articles_title;

-- Revert Embeddings index
ALTER INDEX idx_old_embeddings_embedding_vector_idx RENAME TO embeddings_embedding_vector_idx;

