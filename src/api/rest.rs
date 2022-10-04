// use chrono::{TimeZone, Utc};
// use mongodb::bson::doc;
// use mongodb::bson::oid::ObjectId;
// use rocket::http::Status;
// models::courses::Student
use crate::{models::courses::{Course}, repository::mongodb::MongoRepo};//, CourseJSON, Student, GradeOfStudentInCourse}, repository::mongodb::MongoRepo};
// use futures::io::Cursor;
use mongodb::results::InsertOneResult;
// use mongodb::bson::doc;
// use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

#[post("/create_course", data = "<new_course>")]
pub fn create_course(
    db: &State<MongoRepo>,
    new_course: Json<Course>,
) -> Result<Json<InsertOneResult>, Status> {
    let inserted_course = db.create_course_json(new_course.into_inner());
    match inserted_course {
        Ok(inserted_course) => Ok(Json(inserted_course)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/insert_new_student/<course_name>/<group>/<student_id>")]
pub fn insert_new_student(
    db: &State<MongoRepo>,
    course_name: String,
    group: String,
    student_id: String,
) -> Result<String, Status> {
    let course_response = db.insert_new_student(course_name, group, student_id);
    match course_response {
        Ok(_course_response) => Ok("Student was inserted".to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[put("/change_grade_of_student/<course_name>/<group>/<student_id>/<new_grade>")]
pub fn change_grade_of_student(
    db: &State<MongoRepo>,
    course_name: String,
    group: String,
    student_id: String,
    new_grade: String,
) -> Result<String, Status> {
    let course_response = db.change_grade_of_student(course_name, group, student_id, new_grade);
    match course_response {
        Ok(_course_response) => Ok("Grade was changed".to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/delete_student_on_group/<course_name>/<group>/<student_id>")]
pub fn delete_student_on_group(
    db: &State<MongoRepo>,
    course_name: String,
    group: String,
    student_id: String,
) -> Result<String, Status> {
    let course_response = db.delete_student_on_group(course_name, group, student_id);
    match course_response {
        Ok(_course_response) => Ok("Student was deleted".to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}

// #[get("/get_student_grade/<course_name>/<group>/<student_id>")]
// pub fn get_student_grade(
//     db: &State<MongoRepo>,
//     course_name: String,
//     group: String,
//     student_id: String,
// // ) -> Json<GradeOfStudentInCourse> {
// //     let student_response = db.get_student_grade_on_course(course_name, group, student_id).unwrap();
// //     println!("{>>>>>>>> xd{:?}", student_response);
// //     let student_response_json = Json(student_response);
// //     student_response_json   ;

// //     // let ans = match student_response_json {
// //     //     Ok(response) => response,
// //     //     Err(_) => return Err("Student was not found".into()),
// //     // };

// //     // return ans;
// }