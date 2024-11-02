-- Drop triggers
DROP TRIGGER IF EXISTS update_collections_updated_at ON collections;
DROP TRIGGER IF EXISTS update_articles_updated_at ON articles;
DROP TRIGGER IF EXISTS update_article_search_metadata_updated_at ON article_search_metadata;
DROP TRIGGER IF EXISTS update_embeddings_updated_at ON embeddings;

-- Drop trigger function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes
DROP INDEX IF EXISTS idx_embeddings_chunk;
DROP INDEX IF EXISTS idx_embeddings_keywords;
DROP INDEX IF EXISTS idx_embeddings_bullet_points;
DROP INDEX IF EXISTS idx_embeddings_summary;
DROP INDEX IF EXISTS idx_embeddings_type;
DROP INDEX IF EXISTS idx_embeddings_source_id;

DROP INDEX IF EXISTS idx_article_chunks_is_title;
DROP INDEX IF EXISTS idx_article_chunks_content_search;
DROP INDEX IF EXISTS idx_article_chunks_article_id;

DROP INDEX IF EXISTS idx_article_search_metadata_summary_search;
DROP INDEX IF EXISTS idx_article_search_metadata_keywords;
DROP INDEX IF EXISTS idx_article_search_metadata_bullet_points;
DROP INDEX IF EXISTS idx_article_search_metadata_article_id;

DROP INDEX IF EXISTS idx_articles_title_search;
DROP INDEX IF EXISTS idx_articles_content_search;
DROP INDEX IF EXISTS idx_articles_title;
DROP INDEX IF EXISTS idx_articles_slug;
DROP INDEX IF EXISTS idx_articles_collection_id;

DROP INDEX IF EXISTS idx_collections_name;
DROP INDEX IF EXISTS idx_collections_slug;

-- Drop tables
DROP TABLE IF EXISTS embeddings;
DROP TABLE IF EXISTS article_chunks;
DROP TABLE IF EXISTS article_search_metadata;
DROP TABLE IF EXISTS articles;
DROP TABLE IF EXISTS collections;

-- Drop enum type
DROP TYPE IF EXISTS embedding_type;

-- Drop extensions (optional - might be used by other parts of the application)
-- DROP EXTENSION IF EXISTS vector;
-- DROP EXTENSION IF EXISTS "uuid-ossp";