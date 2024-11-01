#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ChatSession {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub context: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ChatMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub content: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub referenced_article_ids: Vec<Uuid>,
}
