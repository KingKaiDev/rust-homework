#[derive(Debug, Clone)]
pub struct Course {
    pub name: String,
    pub schedule: Vec<String>,
    pub course_code: String,
    pub enrolled_students: u32,
}
