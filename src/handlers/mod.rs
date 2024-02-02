use axum::{
    extract::State as AxumState, Json as JsonAxum
};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::AppState;
use crate::handlers::questions::HandleError;
use crate::models::{Answer, AnswerId, Question, QuestionId};

pub mod questions;

pub async fn health_check() -> impl IntoResponse {
    "OK+".to_string()
}

impl IntoResponse for questions::HandleError {
    fn into_response(self) -> Response {
        match self {
            HandleError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            HandleError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

pub async fn create_question(
    AxumState(AppState { questions_dao, ..  }): AxumState<AppState>,
    JsonAxum(questions): JsonAxum<Question>
) -> Result<impl IntoResponse, impl IntoResponse> {
    questions::create_question(questions, questions_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn read_questions(
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    questions::read_questions(questions_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn delete_question(
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
    JsonAxum(question_uuid): JsonAxum<QuestionId>
) -> Result<impl IntoResponse, impl IntoResponse> {
    questions::delete_question(question_uuid, questions_dao.as_ref()).await
}

pub async fn create_answer(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(answer): JsonAxum<Answer>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    questions::create_answer(answer, answers_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn read_answers(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(question_uuid): JsonAxum<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    questions::read_answers(question_uuid, answers_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn delete_answer(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(answer_uuid): JsonAxum<AnswerId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    questions::delete_answer(answer_uuid, answers_dao.as_ref()).await
}