use crate::models::course::Course;

pub fn display_course(course: &Course) {
    println!("Name: {:?}", course);
    println!("---------------------------");
}
