mod controllers;
mod database;
mod models;
mod utils;
mod views;

use controllers::course_controller::CourseController;
use controllers::student_controller::StudentController;
use database::course_db::CourseDB;
use database::student_db::StudentDB;
use models::course::Course;
use models::student::Student;
use std::io::{self, Write};
use utils::validation;
use views::course_view;
use views::student_view;

fn main() {
    let student_db = StudentDB::new();
    let mut student_controller = StudentController::new(student_db);

    let course_db = CourseDB::new();
    let mut course_controller = CourseController::new(course_db);

    loop {
        print_menu();
        let command = read_line();

        match command.trim() {
            "1" => add_student(&mut student_controller),
            "2" => add_course(&mut course_controller),
            "3" => display_students(&student_controller),
            "4" => display_courses(&course_controller),
            "q" => break,
            _ => println!("Invalid command. Please try again."),
        }
    }
}

fn print_menu() {
    println!("Menu:");
    println!("1. Add Student");
    println!("2. Add Course");
    println!("3. Display Students");
    println!("4. Display Courses");
    println!("q. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn add_student(student_controller: &mut StudentController) {
    println!("Enter student information:");

    print!("Name: ");
    io::stdout().flush().unwrap();
    let name = read_line().trim().to_string();

    print!("Gender: ");
    io::stdout().flush().unwrap();
    let gender = read_line().trim().to_string();

    print!("Age: ");
    io::stdout().flush().unwrap();
    let age: u32 = read_line().trim().parse().unwrap();

    print!("Grade: ");
    io::stdout().flush().unwrap();
    let grade = read_line().trim().to_string();

    print!("Class: ");
    io::stdout().flush().unwrap();
    let class = read_line().trim().to_string();

    print!("Student ID: ");
    io::stdout().flush().unwrap();
    let student_id = read_line().trim().to_string();

    let student = Student {
        name,
        gender,
        age,
        grade,
        class,
        student_id,
    };

    if validation::validate_student(&student) {
        student_controller.add_student(student);
        println!("Student added successfully.");
    } else {
        println!("Invalid student information. Please try again.");
    }
}

fn add_course(course_controller: &mut CourseController) {
    println!("Enter course information:");

    print!("Name: ");
    io::stdout().flush().unwrap();
    let name = read_line().trim().to_string();

    print!("Course Code: ");
    io::stdout().flush().unwrap();
    let course_code = read_line().trim().to_string();

    let course = Course {
        name,
        schedule: Vec::new(),
        course_code,
        enrolled_students: 0,
    };

    if validation::validate_course(&course) {
        course_controller.add_course(course);
        println!("Course added successfully.");
    } else {
        println!("Invalid course information. Please try again.");
    }
}

fn display_students(student_controller: &StudentController) {
    let students = student_controller.get_all_students();

    for student in students {
        student_view::display_student(&student);
    }
}

fn display_courses(course_controller: &CourseController) {
    let courses = course_controller.get_all_courses();

    for course in courses {
        course_view::display_course(&course);
    }
}
