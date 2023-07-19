pub mod controllers;
pub mod database;
pub mod models;
pub mod utils;
pub mod views;

pub use controllers::course_controller::CourseController;
pub use controllers::student_controller::StudentController;
pub use database::course_db::CourseDB;
pub use database::student_db::StudentDB;
pub use models::course::Course;
pub use models::student::Student;
pub use utils::validation;
pub use views::course_view;
pub use views::student_view;
