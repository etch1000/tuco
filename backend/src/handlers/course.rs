use crate::db_access::course::*;
use crate::models::course::{CreatCourse, UpdateCourse};
use crate::errors::TutorError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<u32>) -> Result<HttpResponse, TutorError>> {
    let tutor_id = params.into_inner();

    get_courses_for_tutor_db(&app_state.db, tutor_id).await.map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(app_state: web::Data<AppState>, params: web::Path<(u32, u32)>) -> Result<HttpResponse, TutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let response: Result<HttpResponse, TutorError> = get_course_details_db(&app_state.db, tutor_id, course_id).await.map(|courses| HttpResponse::Ok().json(courses));

    response
}

pub async fn post_new_courses(course: web::Json<CreateCourse>, app_state: web::Data<AppState>) -> Result<HttpResponse, TutorError> {
    let response: Result<HttpResponse, TutorError> = post_new_courses_db(&app_state.db, course.into()).await.map(|courses| HttpResponse::Ok().json(courses));

    response
}

pub async fn update_course_details(app_state: web::Data<AppState>, course: web::Json<UpdateCourse>, params: web::Path<(u32, u32)>) -> Result<HttpResponse, TutorError> {
    let (tutor_id, course_id) = params.into_inner();

    update_course_details_db(&app_state.db, tutor_id, course_id, course.into()).await.map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn delete_course(app_state: web::Data<AppState>, params: web::Path<(u32, u32)>) -> Result<HttpResponse, TutorError> {
    let (tutor_id, course_id) = params.into_inner();
    
    delete_course_db(&app_state.db, tutor_id, course_id).await.map(|resp| HttpResponse::Ok().json(resp))
}


