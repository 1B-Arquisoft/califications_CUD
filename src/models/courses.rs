use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
// use serde::Deserialize;
// use rocket_contrib::json::Json;
#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: String,
    pub grade: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub course_name: String,
    pub teacher_name: String,
    pub teacher_id: String,
    pub group: String,
    pub students_in_course: Vec<Student>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseJSON {
    pub id: Option<String>,
    pub course_name: String,
    pub teacher_name: String,
    pub teacher_id: String,
    pub group: String,
    pub students_in_course: Vec<Student>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseNameAndGroupFilter {
    pub course_name: String,
    pub group: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GradeOfStudentInCourse {
    pub course_name: String,
    pub students_in_course: Vec<Student>
}

impl From<Course> for CourseJSON {
    fn from(course: Course) -> Self {
        CourseJSON {
            id: course.id.map(|id| id.to_hex()),
            course_name: course.course_name,
            teacher_name: course.teacher_name,
            teacher_id: course.teacher_id,
            group: course.group,
            students_in_course: course.students_in_course,
        }
    }

}
    
