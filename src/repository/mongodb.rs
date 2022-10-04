use std::env;
extern crate dotenv;
use dotenv::dotenv;
// use futures::io::Cursor;
// use mongodb::options;
// use mongodb::options::FindOneOptions;
// use mongodb::options::FindOptions;
// use rocket::serde::json::Json;

use crate::models::courses::Course;
// use crate::models::courses::CourseJSON;
// use crate::models::courses::GradeOfStudentInCourse;
// use crate::rocket{serde::json::Json};

use mongodb::{
    bson::{doc, extjson::de::Error}, //, Document}, //modify hereo id::ObjectId
    results::{InsertOneResult, UpdateResult}, //DeleteResult,
    sync::{Client, Collection},
};

// use futures::stream::TryStreamExt;
// use mongodb::{options::FindOptions};

pub struct MongoRepo {
    courses_collection: Collection<Course>,
    // grades_collection: Collection<GradeOfStudentInCourse>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let client_uri =
            env::var("DATABASE_URI").expect("You must set the DATABASE_URI environment var!");
        let client = Client::with_uri_str(client_uri).unwrap();
        let db = client.database("ArquiSoft");
        let actual_courses_collection: Collection<Course> = db.collection("actualCourses");
        // let actual_grades_collection: Collection<GradeOfStudentInCourse> =
        //     db.collection("actualCourses");
        MongoRepo {
            courses_collection: actual_courses_collection,
            // grades_collection: actual_grades_collection,
        }
    }

    pub fn create_course_json(&self, course: Course) -> Result<InsertOneResult, Error> {
        let course_creation_document = Course {
            id: None,
            course_name: course.course_name,
            teacher_name: course.teacher_name,
            teacher_id: course.teacher_id,
            group: course.group,
            students_in_course: course.students_in_course,
        };

        let inserted_course = self
            .courses_collection
            .insert_one(course_creation_document, None)
            .ok()
            .expect("Error inserting course");

        Ok(inserted_course)
    }

    pub fn insert_new_student(
        &self,
        course_name: String,
        group: String,
        student_id: String,
    ) -> Result<UpdateResult, Error> {
        let update_result = self
            .courses_collection
            .update_one(
                doc! {
                    "course_name": course_name,
                    "group": group
                },
                doc! {
                    "$push": {
                        "students_in_course": {
                            "student_id": student_id,
                            "grade": "0",
                        }
                    }
                },
                None,
            )
            .ok()
            .expect("Error updating course");

        Ok(update_result)
    }

    pub fn change_grade_of_student(
        &self,
        course_name: String,
        group: String,
        student_id: String,
        new_grade: String,
    ) -> Result<UpdateResult, Error> {
        let update_result = self
            .courses_collection
            .update_one(
                doc! {
                    "course_name": course_name,
                    "group": group,
                    "students_in_course.student_id": student_id
                },
                doc! {
                    "$set": {
                        "students_in_course.$.grade": new_grade
                    }
                },
                None,
            )
            .ok()
            .expect("Error updating course");

        Ok(update_result)
    }

    pub fn delete_student_on_group(
        &self,
        course_name: String,
        group: String,
        student_id: String,
    ) -> Result<UpdateResult, Error> {
        let update_result = self
            .courses_collection
            .update_one(
                doc! {
                    "course_name": course_name,
                    "group": group
                },
                doc! {
                    "$pull": {
                        "students_in_course": {
                            "student_id": student_id
                        }
                    }
                },
                None,
            )
            .ok()
            .expect("Error updating course");

        Ok(update_result)
    }

    // pub fn get_student_grade_on_course(
    //     &self,
    //     course_name: String,
    //     group: String,
    //     student_id: String,
    // ) -> Result<GradeOfStudentInCourse, Error> {
    //     let filter = doc! {
    //         "course_name": course_name.clone(),
    //         "students_in_course.id": student_id.clone(),
    //         "group": group.clone(),
    //     };
    //     let find_projection_filter = doc! {
    //         "students_in_course":{
    //             "$elemMatch": {
    //                 "id": student_id,
    //             }
    //         },
    //         "course_name": course_name
    //     };
    //     let find_options = FindOneOptions::builder()
    //         .projection(find_projection_filter)
    //         .build();
    //     let cursor = self.grades_collection.find_one(filter, find_options);
    //     Ok(cursor.unwrap().unwrap())
    // }

    // let mut course = match cursor {
    //     Ok(course) => course,
    //     Err(e) => {
    //         println!("Error getting course")
    //     }
    // }
}
