-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS vector;

-- Create embedding_type enum
CREATE TYPE embedding_type AS ENUM ('summary', 'bullet_points', 'keywords', 'chunk');

-- Create collections table
CREATE TABLE collections (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    slug TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create articles table
CREATE TABLE articles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    collection_id UUID NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create article_search_metadata table
CREATE TABLE article_search_metadata (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    summary TEXT NOT NULL,
    bullet_points TEXT[] NOT NULL,
    keywords TEXT[] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create article_chunks table
CREATE TABLE article_chunks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_title BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create embeddings table
CREATE TABLE embeddings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_id UUID NOT NULL,
    embedding_type embedding_type NOT NULL,
    embedding vector(384) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for collections
CREATE UNIQUE INDEX idx_collections_slug ON collections(slug);
CREATE INDEX idx_collections_name ON collections(name);

-- Create indexes for articles
CREATE INDEX idx_articles_collection_id ON articles(collection_id);
CREATE UNIQUE INDEX idx_articles_slug ON articles(slug);
CREATE INDEX idx_articles_title ON articles(title);

-- Create text search indexes
CREATE INDEX idx_articles_content_search ON articles USING GIN (to_tsvector('english', content));
CREATE INDEX idx_articles_title_search ON articles USING GIN (to_tsvector('english', title));

-- Create indexes for article_search_metadata
CREATE INDEX idx_article_search_metadata_article_id ON article_search_metadata(article_id);
CREATE INDEX idx_article_search_metadata_bullet_points ON article_search_metadata USING gin(bullet_points);
CREATE INDEX idx_article_search_metadata_keywords ON article_search_metadata USING gin(keywords);
CREATE INDEX idx_article_search_metadata_summary_search 
    ON article_search_metadata USING GIN (to_tsvector('english', summary));

-- Create indexes for article_chunks
CREATE INDEX idx_article_chunks_article_id ON article_chunks(article_id);
CREATE INDEX idx_article_chunks_content_search ON article_chunks USING GIN (to_tsvector('english', content));
CREATE INDEX idx_article_chunks_is_title ON article_chunks(is_title);

-- Create indexes for embeddings
CREATE INDEX idx_embeddings_source_id ON embeddings(source_id);
CREATE INDEX idx_embeddings_type ON embeddings(embedding_type);

-- Create specialized indexes for each embedding type
CREATE INDEX idx_embeddings_summary 
ON embeddings USING ivfflat (embedding vector_l2_ops)
WITH (lists = 100)
WHERE embedding_type = 'summary';

CREATE INDEX idx_embeddings_bullet_points 
ON embeddings USING ivfflat (embedding vector_l2_ops)
WITH (lists = 100)
WHERE embedding_type = 'bullet_points';

CREATE INDEX idx_embeddings_keywords 
ON embeddings USING ivfflat (embedding vector_l2_ops)
WITH (lists = 100)
WHERE embedding_type = 'keywords';

CREATE INDEX idx_embeddings_chunk 
ON embeddings USING ivfflat (embedding vector_l2_ops)
WITH (lists = 100)
WHERE embedding_type = 'chunk';

-- Create trigger function for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_collections_updated_at
    BEFORE UPDATE ON collections
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_articles_updated_at
    BEFORE UPDATE ON articles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_article_search_metadata_updated_at
    BEFORE UPDATE ON article_search_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_embeddings_updated_at
    BEFORE UPDATE ON embeddings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();