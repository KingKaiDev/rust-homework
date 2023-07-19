use std::collections::HashMap;
use crate::models::student::Student;

pub struct StudentDB {
    students: HashMap<String, Student>,
}

impl StudentDB {
    pub fn new() -> StudentDB {
        StudentDB {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.student_id.clone(), student);
    }

    pub fn get_student(&self, student_id: &str) -> Option<Student> {
        self.students.get(student_id).cloned()
    }

    pub fn update_student(&mut self, student_id: &str, updated_student: Student) -> bool {
        if self.students.contains_key(student_id) {
            self.students.insert(student_id.to_string(), updated_student);
            true
        } else {
            false
        }
    }

    pub fn delete_student(&mut self, student_id: &str) -> bool {
        if self.students.contains_key(student_id) {
            self.students.remove(student_id);
            true
        } else {
            false
        }
    }

    pub fn get_all_students(&self) -> Vec<Student> {
        self.students.values().cloned().collect()
    }
}
