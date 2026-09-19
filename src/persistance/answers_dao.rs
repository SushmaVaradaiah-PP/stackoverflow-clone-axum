use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let question_uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!(
                "Could not parse answer UUID: {}",
                answer.question_uuid
            ))
        })?;

        let record = sqlx::query_as::<_, AnswerDetail>(
            r#"INSERT INTO answers (question_uuid,content) VALUES ($1,$2) RETURNING answer_uuid::text, question_uuid::text, content, created_at::text"#,
        )
        .bind(question_uuid)
        .bind(&answer.content)
        .fetch_one(&self.db)
        .await
        .map_err(|e: sqlx::Error| match e {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
                        return DBError::InvalidUUID(format!("Invalid question UUID: {}",answer.question_uuid));
                    }
                }
                DBError::Other(Box::new(e))
            }
            e => DBError::Other(Box::new(e))
        })?;

        Ok(record)
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse answer UUID: {}", answer_uuid))
        })?;

        let result = sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        if result.rows_affected() == 0 {
            return Err(DBError::InvalidUUID(format!(
                "No answer found with UUID: {}",
                answer_uuid
            )));
        }

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        })?;

        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|r| AnswerDetail {
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.clone(),
                created_at: r.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
