use axum::async_trait;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswerDaoImpl {
    db: PgPool
}

impl AnswerDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswerDaoImpl {
            db
        }
    }
}

#[async_trait]
impl AnswersDao for AnswerDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("UUID has invalid format: {}",
            answer.question_uuid))
        })?;

        let record = sqlx::query!("insert into answers (question_uuid, content) values ($1, $2) returning *",
                uuid,
            answer.content
            )
            .fetch_one(&self.db)
            .await
            .map_err(|e: sqlx::Error| match e {
                sqlx::Error::Database(e) => {
                    if let Some(code) = e.code() {
                        if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
                            return DBError::InvalidUUID(format!(
                                "Invalid question UUID: {}",
                                answer.question_uuid
                            ))
                        }
                    }
                    DBError::Other(Box::new(e))
                }
                e => DBError::Other(Box::new(e))
            })?;


        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(&answer_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("UUID has invalid format: {}", answer_uuid))
        })?;

        sqlx::query!("delete from answers where answer_uuid=$1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;
        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::parse_str(&question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("UUID has invalid format: {}", question_uuid))
        })?;

        let records = sqlx::query!("select * from answers where question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|r| AnswerDetail {
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.to_string(),
                created_at: r.created_at.to_string(),
            })
            .collect();
        Ok(answers)
    }
}