use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    // 为什么是Option类型，就是因为它可以是空的
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(value: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: value.name.clone(),
            picture_url: value.picture_url.clone(),
            profile: value.profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(value: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: value.name.clone(),
            picture_url: value.picture_url.clone(),
            profile: value.profile.clone(),
        }
    }
}
