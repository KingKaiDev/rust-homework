use crate::database::course_db::CourseDB;
use crate::models::course::Course;

pub struct CourseController {
    course_db: CourseDB,
}

impl CourseController {
    pub fn new(course_db: CourseDB) -> CourseController {
        CourseController { course_db }
    }

    pub fn add_course(&mut self, course: Course) {
        self.course_db.add_course(course);
    }

    pub fn get_course(&self, course_code: &str) -> Option<Course> {
        self.course_db.get_course(course_code)
    }

    pub fn update_course(&mut self, course_code: &str, updated_course: Course) -> bool {
        self.course_db.update_course(course_code, updated_course)
    }

    pub fn delete_course(&mut self, course_code: &str) -> bool {
        self.course_db.delete_course(course_code)
    }

    pub fn get_all_courses(&self) -> Vec<Course> {
        self.course_db.get_all_courses()
    }
}
