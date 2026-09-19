use crate::models::{DBError, Question, QuestionDetail};
use async_trait::async_trait;
use sqlx::PgPool;

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
        Self { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query_as::<_, QuestionDetail>(
            r#"INSERT INTO questions(title,description) VALUES ($1,$2) RETURNING question_uuid::text,title,description,created_at::text"#,
        )
        .bind(&question.title)
        .bind(&question.description)
        .fetch_one(&self.db)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(record)
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        })?;

        let result = sqlx::query!("DELETE FROM questions WHERE question_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        if result.rows_affected() == 0 {
            return Err(DBError::InvalidUUID(format!(
                "No answer found with UUID: {}",
                question_uuid
            )));
        }

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!("SELECT * FROM questions")
            .fetch_all(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        let questions = records
            .iter()
            .map(|r| QuestionDetail {
                question_uuid: r.question_uuid.to_string(),
                title: r.title.clone(),
                description: r.description.clone(),
                created_at: r.created_at.to_string(),
            })
            .collect();

        Ok(questions)
    }
}
