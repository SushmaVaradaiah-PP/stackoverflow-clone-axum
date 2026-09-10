use crate::models::*;
use axum::{Json, response::IntoResponse};

// CRUD for questions
pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    todo!()
}

// CRUD for answers
pub async fn create_answer(Json(answer): Json<Answer>) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_answers(Json(question_id): Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_answer(Json(answer_id): Json<AnswerId>) {
    todo!()
}
