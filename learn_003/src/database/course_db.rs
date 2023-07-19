use crate::models::course::Course;
use std::collections::HashMap;

pub struct CourseDB {
    courses: HashMap<String, Course>,
}

impl CourseDB {
    pub fn new() -> CourseDB {
        CourseDB {
            courses: HashMap::new(),
        }
    }

    pub fn add_course(&mut self, course: Course) {
        self.courses.insert(course.course_code.clone(), course);
    }

    pub fn get_course(&self, course_code: &str) -> Option<Course> {
        self.courses.get(course_code).cloned()
    }

    pub fn update_course(&mut self, course_code: &str, updated_course: Course) -> bool {
        if self.courses.contains_key(course_code) {
            self.courses.insert(course_code.to_string(), updated_course);
            true
        } else {
            false
        }
    }

    pub fn delete_course(&mut self, course_code: &str) -> bool {
        if self.courses.contains_key(course_code) {
            self.courses.remove(course_code);
            true
        } else {
            false
        }
    }

    pub fn get_all_courses(&self) -> Vec<Course> {
        self.courses.values().cloned().collect()
    }
}
