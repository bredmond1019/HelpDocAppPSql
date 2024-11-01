-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS uuid-ossp;

CREATE TABLE collections (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    slug VARCHAR(255) NOT NULL,
    helpscout_collection_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    paragraph_description TEXT,
    bullet_points TEXT,
    keywords TEXT,
    paragraph_description_embedding vector(384),
    bullet_points_embedding vector(384),
    keywords_embedding vector(384)
);

CREATE TABLE articles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    collection_id UUID NOT NULL REFERENCES collections(id),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    html_content TEXT,
    markdown_content TEXT,
    version INTEGER NOT NULL DEFAULT 0,
    last_edited_by VARCHAR(255),
    helpscout_collection_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    helpscout_article_id VARCHAR,
    paragraph_description TEXT,
    bullet_points TEXT[],
    keywords TEXT[],
    paragraph_description_embedding vector(384),
    bullet_points_embedding vector(384),
    keywords_embedding vector(384)
);

CREATE TABLE article_chunks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID NOT NULL REFERENCES articles(id),
    content TEXT NOT NULL,
    is_title BOOLEAN NOT NULL,
    embedding_id UUID
);

CREATE TABLE embeddings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID NOT NULL REFERENCES articles(id),
    embedding_vector vector(384) NOT NULL
);

-- Add indexes for performance
CREATE INDEX idx_collections_slug ON collections(slug);
CREATE INDEX idx_articles_slug ON articles(slug);
CREATE INDEX idx_articles_collection_id ON articles(collection_id);
CREATE INDEX idx_article_chunks_article_id ON article_chunks(article_id);
CREATE INDEX idx_embeddings_article_id ON embeddings(article_id);

-- Add vector indexes for similarity search
CREATE INDEX ON articles USING ivfflat (paragraph_description_embedding vector_cosine_ops);
CREATE INDEX ON articles USING ivfflat (bullet_points_embedding vector_cosine_ops);
CREATE INDEX ON articles USING ivfflat (keywords_embedding vector_cosine_ops);
CREATE INDEX ON embeddings USING ivfflat (embedding_vector vector_cosine_ops);



-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
