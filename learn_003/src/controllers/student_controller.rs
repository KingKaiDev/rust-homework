use crate::database::student_db::StudentDB;
use crate::models::student::Student;

pub struct StudentController {
    student_db: StudentDB,
}

impl StudentController {
    pub fn new(student_db: StudentDB) -> StudentController {
        StudentController { student_db }
    }

    pub fn add_student(&mut self, student: Student) {
        self.student_db.add_student(student);
    }

    pub fn get_student(&self, student_id: &str) -> Option<Student> {
        self.student_db.get_student(student_id)
    }

    pub fn update_student(&mut self, student_id: &str, updated_student: Student) -> bool {
        self.student_db.update_student(student_id, updated_student)
    }

    pub fn delete_student(&mut self, student_id: &str) -> bool {
        self.student_db.delete_student(student_id)
    }

    pub fn get_all_students(&self) -> Vec<Student> {
        self.student_db.get_all_students()
    }
}
