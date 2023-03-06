use crate::db_access::tutor::*;
use crate::errors::TutorError;
use crate::models::tutor::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, TutorError> {
    get_all_tutors_db(&app_state.db).await.map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(app_state: web::Data<AppState>, params: web::Path<(u32, )>) -> Result<HttpResponse, TutorError> {
    let (tutor_id, ) = params.into_inner();

    get_tutor_details_db(&app_state.db, tutor_id).await.map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn post_new_tutor(app_state: web::Data<AppState>, tutor: web::Json<CreateTutor>) -> Result<HttpResponse, TutorError> {
    post_new_tutor_db(&app_state.db, CreateTutor::from(tutor)).await.map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(app_state: web::Data<AppState>, tutor: web::Json<UpdateTutor>, params: web::Path<(u32,)) -> Result<HttpResponse, TutorError> {
    let (tutor_id, ) = params.into_inner();

    update_tutor_details_db(&app_state.db, tutor_id).await.map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(app_state: web::Data<AppState>, params: web::Path<(u32,)>) -> Result<HttpResponse, TutorError> {
    let (tutor_id, ) = params.into_inner();

    delete_tutor_db(&app_state.db, tutor_id).await.map(|t| HttpResponse::Ok().json(t))
}
