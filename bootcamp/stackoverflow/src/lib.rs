mod handlers;
mod models;
mod persistance;

use handlers::*;
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::{pool::Pool, Postgres};
use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

pub async fn run(pool: Pool<Postgres>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    let app = app(pool);
    axum::serve(listener, app).await.unwrap();
}

fn app(pool: Pool<Postgres>) -> Router {
    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool));
    let state = AppState {
        questions_dao,
        answers_dao,
    };

    Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use sqlx::PgPool;

    /// An e2e test of our app
    #[sqlx::test]
    async fn e2e(pool: PgPool) -> sqlx::Result<()> {
        let app = app(pool.clone());
        let server = TestServer::new(app).unwrap();

        let test_question = Question {
            title: "Toto title".to_string(),
            description: "Toto description".to_string(),
        };

        // Create a question
        let create_question_req = server.post("/question").json(&test_question);
        let created_question = create_question_req.await.json::<QuestionDetail>();
        assert_eq!(created_question.title, test_question.title);
        assert_eq!(created_question.description, test_question.description);
        let qid = QuestionId {
            question_uuid: created_question.question_uuid.clone(),
        };

        // Get questions in db
        let questions_in_db = server.get("/questions").await.json::<Vec<QuestionDetail>>();
        assert!(!questions_in_db.is_empty());
        assert_eq!(&created_question, questions_in_db.first().unwrap());

        // Create answer
        let test_answer = Answer {
            question_uuid: created_question.question_uuid.clone(),
            content: "Answer content".to_string(),
        };
        let create_answer_req = server.post("/answer").json(&test_answer);
        let created_answer = create_answer_req.await.json::<AnswerDetail>();
        assert_eq!(created_answer.question_uuid, test_answer.question_uuid);
        assert_eq!(created_answer.content, test_answer.content);

        // Get answers in db
        let answers_in_db = server
            .get("/answers")
            .json(&qid)
            .await
            .json::<Vec<AnswerDetail>>();
        assert!(!answers_in_db.is_empty());
        assert_eq!(&created_answer, answers_in_db.first().unwrap());

        // Delete answer
        let aid = AnswerId {
            answer_uuid: created_answer.answer_uuid.clone(),
        };
        let delete_answer_req = server.delete("/answer").json(&aid);
        delete_answer_req.expect_success().await;

        // Get answers in db
        let answers_in_db = server
            .get("/answers")
            .json(&qid)
            .await
            .json::<Vec<AnswerDetail>>();

        assert!(answers_in_db.is_empty());

        // Delete question
        let qid = QuestionId {
            question_uuid: created_question.question_uuid.clone(),
        };
        let delete_question_req = server.delete("/question").json(&qid);
        delete_question_req.expect_success().await;

        // Get questions in db
        let questions_in_db = server.get("/questions").await.json::<Vec<QuestionDetail>>();
        assert!(questions_in_db.is_empty());

        // Create an answer to a deleted question which should provide a failure
        let create_answer_req = server.post("/answer").json(&test_answer);
        let created_answer = create_answer_req.expect_failure().await;
        assert_eq!(StatusCode::BAD_REQUEST, created_answer.status_code());

        Ok(())
    }

    /// Code for debugging
    #[allow(dead_code)]
    async fn print_db_state(pool: &PgPool) {
        let records = sqlx::query!(r"SELECT * FROM questions")
            .fetch_all(pool)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))
            .unwrap();

        // Iterate over `records` and map each record to a `QuestionDetail` type
        let questions: Vec<QuestionDetail> = records
            .iter()
            .map(|rec| QuestionDetail {
                question_uuid: rec.question_uuid.to_string(),
                title: rec.title.to_string(),
                description: rec.description.to_string(),
                created_at: rec.created_at.to_string(),
            })
            .collect();

        println!("questions: {:?}", questions);
    }
}
