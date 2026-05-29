use dialoguer::{Input, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::exit};

use super::saving::{Save, save_subject};

#[derive(Serialize, Deserialize, Clone)]
pub struct Subject {
    pub name: String,
    pub value: f32,
}

// Prompt users to add a subject
pub fn prompt_subject(save: bool) -> Subject {
    let subject_name_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name of subject:".to_string())
        .interact_text()
        .unwrap();

    let subject_weight_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Subject weight:".to_string())
        .interact_text()
        .unwrap();

    let subject_weight_float = match subject_weight_input.parse::<f32>() {
        Ok(v) => v,
        Err(_) => {
            println!("ERROR: The weight that you input must be a number, decimals allowed.");
            exit(1);
        }
    };

    let subject = Subject {
        name: subject_name_input,
        value: subject_weight_float,
    };

    if save {
        // save logic
        save_subject(subject.clone());
    }

    subject
}

// Prompt users to enter grades based on subjects

pub fn prompt_grades(subjects: Vec<Subject>) -> HashMap<String, f32> {
    let mut grades: HashMap<String, f32> = HashMap::new();

    // prompt for each one and append it to a vector
    for subject in subjects {
        let prompt = format!("{} Grade (number)", subject.name);

        let grade_input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .unwrap();

        let grade_as_float = match grade_input.parse::<f32>() {
            Ok(v) => v,
            Err(_) => {
                println!("ERROR: The grade that you input must be a number, decimals allowed.");
                exit(1);
            }
        };

        grades.insert(subject.name, grade_as_float);
    }

    // return the vector of grades, their position should correspond to the subject's position in the array of subjects
    grades
}

// returns subjects with a corresponding grade as a hashmap (to make it easier to print)
pub struct SubjectWithGrade {
    pub subject: Subject,
    pub grade: f32,
}

pub fn subjects_with_grades(save: Save) -> Vec<SubjectWithGrade> {
    let subjects: Vec<Subject> = save.subjects.into_values().collect();
    let grades = save.grades;
    let mut subject_and_grades: Vec<SubjectWithGrade> = vec![];

    // go through each subject, find its grade and add it to the subject_and_grades vector
    for subject in subjects {
        let grade_for_subject = grades.get(&subject.name).unwrap_or_else(|| {
            println!(
                "ERROR: Grade missing for subject {}, please run the enter grades function.",
                subject.name
            );
            exit(1);
        });

        let subject_with_grade = SubjectWithGrade {
            subject,
            grade: *grade_for_subject,
        };

        subject_and_grades.push(subject_with_grade);
    }

    subject_and_grades
}

// Quebec Secondary III default (2026)
pub fn default_subjects() -> Vec<Subject> {
    vec![
        Subject {
            name: "Math".to_string(),
            value: 6.0,
        },
        Subject {
            name: "English".to_string(),
            value: 6.0,
        },
        Subject {
            name: "French".to_string(),
            value: 6.0,
        },
        Subject {
            name: "Science".to_string(),
            value: 6.0,
        },
        Subject {
            name: "History".to_string(),
            value: 4.0,
        },
        Subject {
            name: "POP".to_string(),
            value: 4.0,
        },
        Subject {
            name: "Music".to_string(),
            value: 2.0,
        },
        Subject {
            name: "Gym".to_string(),
            value: 2.0,
        },
        Subject {
            name: "Study".to_string(),
            value: 2.0,
        },
    ]
}
