mod api;
mod models;
mod repository;

// use dotenv::dotenv;

use api::rest::{
    create_course, 
    insert_new_student,
    change_grade_of_student,
    delete_student_on_group,
    // get_student_grade,
    // get_courses_by_name,
};
use repository::mongodb::MongoRepo;


#[macro_use] extern crate rocket;

#[get("/")]
fn test_rocket() -> &'static str {
    return "Rocket Service Requested Succesfully. Califications module online!"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
    .manage(db)
    .mount("/", routes![test_rocket])
    .mount("/", routes![create_course])
    .mount("/", routes![insert_new_student])
    .mount("/", routes![change_grade_of_student])
    .mount("/", routes![delete_student_on_group])
    // .mount("/", routes![get_student_grade])
    
    // .mount("/", routes![get_courses_by_name])

    
}
