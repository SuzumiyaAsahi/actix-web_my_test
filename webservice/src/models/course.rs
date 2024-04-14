use crate::errors::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// use crate::models::course::Course;
// sqlx::FromRow 读取数据库表后自动映射
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;
    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        let _ = i32::try_from(course.teacher_id)
            .map_err(|_| MyError::DBError("the teacher id is not right".into()))?;
        let price = course.price;
        match price {
            Some(_) => Ok(CreateCourse {
                teacher_id: course.teacher_id,
                name: course.name.clone(),
                description: course.description.clone(),
                format: course.format.clone(),
                structure: course.structure.clone(),
                duration: course.duration.clone(),
                price: course.price,
                language: course.language.clone(),
                level: course.level.clone(),
            }),
            None => Err(MyError::DBError("the price is none".into())),
        }
    }
}
