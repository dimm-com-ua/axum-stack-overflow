extern crate log;

extern crate pretty_env_logger;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router};
use axum::routing::{delete, get, post};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::handlers::health_check;
use crate::persistance::answers_dao::{AnswerDaoImpl, AnswersDao};
use crate::persistance::questions_dao::{QuestionsDao, QuestionsDaoImpl};

mod handlers;

mod models;
mod persistance;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Sync + Send>,
    pub answers_dao: Arc<dyn AnswersDao + Sync + Send>
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not specified"))
        .await
        .expect("Coulndn't connect to database pool!");

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswerDaoImpl::new(pool.clone()));

    let app_state = AppState {
        questions_dao,
        answers_dao,
    };

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}