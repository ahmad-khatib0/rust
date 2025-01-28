use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: Option<i32>,
    pub course_name: String,
    // NativeDateTime is a chrono data type for storing timestamp information.
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    // This function will convert data from incoming HTTP requests to Rust structs.
    // data from an incoming request body is made available to handler
    // functions through the web::Json<T> extractor.
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}
