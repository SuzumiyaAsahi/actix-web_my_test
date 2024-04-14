// use super::models::Course;
use sqlx::postgres::PgPool;
use std::sync::Mutex;

pub struct Appstate {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    pub db: PgPool,
}
