#[derive(Debug, Queryable)]
pub struct Embedding {
    pub id: Uuid,
    pub article_id: Uuid,
    pub embedding_vector: Vector,
}
