use serde::{Deserialize, Serialize};
use sqlx::types::{time::PrimitiveDateTime, Uuid};
use sqlx::{postgres::PgRow, FromRow, Row};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

impl FromRow<'_, PgRow> for QuestionDetail {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let uuid: Uuid = row.try_get("question_uuid")?;
        let title: String = row.try_get("title")?;
        let description: String = row.try_get("description")?;
        let created_at: PrimitiveDateTime = row.try_get("created_at")?;
        let created_at = format!("{:?}", created_at);
        Ok(QuestionDetail {
            question_uuid: uuid.to_string(),
            title,
            description,
            created_at,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
    pub question_uuid: String,
}

// ----------

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

impl FromRow<'_, PgRow> for AnswerDetail {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let quid: Uuid = row.try_get("question_uuid")?;
        let auid: Uuid = row.try_get("answer_uuid")?;
        let content: String = row.try_get("content")?;
        let created_at: PrimitiveDateTime = row.try_get("created_at")?;
        let created_at = format!("{:?}", created_at);
        Ok(AnswerDetail {
            question_uuid: quid.to_string(),
            answer_uuid: auid.to_string(),
            content,
            created_at,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_uuid: String,
}

// ----------

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
#[allow(dead_code)]
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}
