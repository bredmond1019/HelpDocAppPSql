// File: src/db/schema.rs
use diesel::prelude::*;
use diesel::table;

table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    articles (id) {
        id -> Uuid,
        collection_id -> Uuid,
        title -> Varchar,
        slug -> Varchar,
        html_content -> Nullable<Text>,
        markdown_content -> Nullable<Text>,
        version -> Integer,
        last_edited_by -> Nullable<Varchar>,
        helpscout_collection_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        helpscout_article_id -> Nullable<Varchar>,
        paragraph_description -> Nullable<Text>,
        bullet_points -> Array<Text>,
        keywords -> Array<Text>,
        paragraph_description_embedding -> Nullable<Vector>,
        bullet_points_embedding -> Nullable<Vector>,
        keywords_embedding -> Nullable<Vector>,
    }
}

table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    collections (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        slug -> Varchar,
        helpscout_collection_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        paragraph_description -> Nullable<Text>,
        bullet_points -> Nullable<Text>,
        keywords -> Nullable<Text>,
        paragraph_description_embedding -> Nullable<Vector>,
        bullet_points_embedding -> Nullable<Vector>,
        keywords_embedding -> Nullable<Vector>,
    }
}

table! {
    article_chunks (id) {
        id -> Uuid,
        article_id -> Uuid,
        content -> Text,
        is_title -> Bool,
        embedding_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    embeddings (id) {
        id -> Uuid,
        article_id -> Uuid,
        embedding_vector -> Vector,
    }
}

// New tables for chat functionality
table! {
    chat_sessions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        context -> Jsonb,
    }
}

table! {
    chat_messages (id) {
        id -> Uuid,
        session_id -> Uuid,
        content -> Text,
        role -> Varchar,
        created_at -> Timestamptz,
        referenced_article_ids -> Array<Uuid>,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    collections,
    article_chunks,
    embeddings,
    chat_sessions,
    chat_messages,
);
