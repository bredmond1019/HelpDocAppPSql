// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
    #[diesel(postgres_type(name = "embedding_type"))]
    pub struct EmbeddingType;
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    article_chunks (id) {
        id -> Uuid,
        article_id -> Uuid,
        content -> Text,
        is_title -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    article_search_metadata (id) {
        id -> Uuid,
        article_id -> Uuid,
        summary -> Text,
        bullet_points -> Array<Nullable<Text>>,
        keywords -> Array<Nullable<Text>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    articles (id) {
        id -> Uuid,
        collection_id -> Uuid,
        title -> Text,
        slug -> Text,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    collections (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        slug -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    content_versions (id) {
        id -> Uuid,
        article_id -> Nullable<Uuid>,
        version_number -> Int4,
        markdown_content -> Nullable<Text>,
        #[max_length = 255]
        edited_by -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use super::sql_types::EmbeddingType;

    embeddings (id) {
        id -> Uuid,
        source_id -> Uuid,
        embedding_type -> EmbeddingType,
        embedding -> Vector,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    old_article_chunks (id) {
        id -> Uuid,
        article_id -> Uuid,
        content -> Text,
        is_title -> Bool,
        embedding_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    old_articles (id) {
        id -> Uuid,
        collection_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        html_content -> Nullable<Text>,
        markdown_content -> Nullable<Text>,
        version -> Int4,
        #[max_length = 255]
        last_edited_by -> Nullable<Varchar>,
        #[max_length = 255]
        helpscout_collection_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        helpscout_article_id -> Nullable<Varchar>,
        paragraph_description -> Nullable<Text>,
        bullet_points -> Nullable<Array<Nullable<Text>>>,
        keywords -> Nullable<Array<Nullable<Text>>>,
        paragraph_description_embedding -> Nullable<Vector>,
        bullet_points_embedding -> Nullable<Vector>,
        keywords_embedding -> Nullable<Vector>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    old_collections (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        slug -> Varchar,
        #[max_length = 255]
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

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    old_embeddings (id) {
        id -> Uuid,
        article_id -> Uuid,
        embedding_vector -> Vector,
    }
}

diesel::joinable!(article_chunks -> articles (article_id));
diesel::joinable!(article_search_metadata -> articles (article_id));
diesel::joinable!(articles -> collections (collection_id));
diesel::joinable!(content_versions -> old_articles (article_id));
diesel::joinable!(old_article_chunks -> old_articles (article_id));
diesel::joinable!(old_articles -> old_collections (collection_id));
diesel::joinable!(old_embeddings -> old_articles (article_id));

diesel::allow_tables_to_appear_in_same_query!(
    article_chunks,
    article_search_metadata,
    articles,
    collections,
    content_versions,
    embeddings,
    old_article_chunks,
    old_articles,
    old_collections,
    old_embeddings,
);
