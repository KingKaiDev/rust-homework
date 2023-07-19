use crate::models::student::Student;

pub fn display_student(student: &Student) {
    println!("Name: {:?}", student);
    println!("---------------------------");
}
