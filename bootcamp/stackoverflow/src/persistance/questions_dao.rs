use async_trait::async_trait;
use sqlx::{types::Uuid, PgPool};

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionsDaoImpl { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        // Make a database query to insert a new question.
        // Here is the SQL query:
        // ```
        // INSERT INTO questions ( title, description )
        // VALUES ( $1, $2 )
        // RETURNING *
        // ```
        // If executing the query results in an error, map that error to
        // the`DBError::Other` error and early return from this function.
        let mut record = sqlx::query_as::<_, QuestionDetail>(
            r"
        INSERT INTO questions ( title, description )
        VALUES ( $1, $2 )
        RETURNING *
",
        )
        .bind(&question.title)
        .bind(&question.description)
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(record.pop().unwrap())
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid =
            Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        // Make a database query to delete a question given the question uuid.
        // Here is the SQL query:
        // ```
        // DELETE FROM questions WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.

        sqlx::query(r"DELETE FROM questions WHERE question_uuid = $1")
            .bind(uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        // Make a database query to get all questions.
        // Here is the SQL query:
        // ```
        // SELECT * FROM questions
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        let records = sqlx::query!(r"SELECT * FROM questions")
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        // Iterate over `records` and map each record to a `QuestionDetail` type
        let questions = records
            .iter()
            .map(|rec| QuestionDetail {
                question_uuid: rec.question_uuid.to_string(),
                title: rec.title.to_string(),
                description: rec.description.to_string(),
                created_at: rec.created_at.to_string(),
            })
            .collect();

        Ok(questions)
    }
}
