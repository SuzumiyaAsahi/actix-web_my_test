use crate::errors::MyError;
use crate::state::Appstate;
use crate::{dbaccess::course::*, models::course::CreateCourse};
use actix_web::{web, HttpResponse};

use crate::models::course::UpdateCourse;

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<Appstate>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<Appstate>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_detail(
    app_state: web::Data<Appstate>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<Appstate>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<Appstate>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    // #[ignore]
    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<Appstate> = web::Data::new(Appstate {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test course".into(),
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: Some(0),
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        let resp = post_new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<Appstate> = web::Data::new(Appstate {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_coures_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<Appstate> = web::Data::new(Appstate {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_coures_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<Appstate> = web::Data::new(Appstate {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = get_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong ..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<Appstate> = web::Data::new(Appstate {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let update_course = UpdateCourse {
            name: Some("Course name changed".into()),
            description: Some("This is another test course".into()),
            format: None,
            level: Some("Intermediate".into()),
            price: Some(0),
            duration: None,
            language: Some("Chinese".into()),
            structure: None,
        };

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let update_param = web::Json(update_course);
        let resp = update_course_details(app_state, update_param, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<Appstate> = web::Data::new(Appstate {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK)
    }
    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<Appstate> = web::Data::new(Appstate {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 101));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
