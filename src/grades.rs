use std::collections::HashMap;

use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;

use super::errors::ScoreledgerGradeError;

use super::saving;
use super::subject::Subject;

// Prompt users to enter grades based on subjects
pub fn prompt_grades(
    subjects: Vec<Subject>,
) -> Result<HashMap<String, f64>, ScoreledgerGradeError> {
    let mut grades: HashMap<String, f64> = HashMap::new();

    // prompt for each one and append it to a vector
    for subject in subjects {
        let prompt = format!("{} Grade (number)", subject.name);
        // find any existing grades for the subject in save, if not then ignore it
        let data = saving::get_data()
            .expect("Unexpected Error: Failed to load save data to determine existing grades list");
        let grade = data.grades.get(&subject.name);
        let grade_default: String = match grade {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };

        let grade_input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(grade_default)
            .interact_text()
            .unwrap();

        let grade_as_float = match grade_input.parse::<f64>() {
            Ok(v) => v,
            Err(_) => return Err(ScoreledgerGradeError::NaNGrade),
        };

        grades.insert(subject.name, grade_as_float);
    }

    // return the vector of grades, their position should correspond to the subject's position in the array of subjects
    Ok(grades)
}

pub fn verify_grades(save: &saving::Save) -> Result<(), ScoreledgerGradeError> {
    // convert subjects into vector
    let subjects: Vec<Subject> = save.subjects.clone().into_values().collect();

    if subjects.is_empty() {
        return Err(ScoreledgerGradeError::NoSubjectsForGrades);
    }

    for subject in subjects {
        match save.grades.get(&subject.name) {
            Some(_) => {
                continue;
            }
            None => return Err(ScoreledgerGradeError::GradeMissing(subject.name.clone())),
        };
    }

    Ok(())
}

// returns subjects with a corresponding grade as a hashmap (to make it easier to print)
pub struct SubjectWithGrade {
    pub subject: Subject,
    pub grade: f64,
}

pub fn subjects_with_grades(
    save: &saving::Save,
) -> Result<Vec<SubjectWithGrade>, ScoreledgerGradeError> {
    let subjects: Vec<Subject> = save.subjects.clone().into_values().collect();
    let grades = save.grades.clone();
    let mut subject_and_grades: Vec<SubjectWithGrade> = vec![];

    // go through each subject, find its grade and add it to the subject_and_grades vector
    for subject in subjects {
        let grade_for_subject = match grades.get(&subject.name) {
            Some(v) => *v,
            None => return Err(ScoreledgerGradeError::GradeMissing(subject.name.clone())),
        };

        let subject_with_grade = SubjectWithGrade {
            subject,
            grade: grade_for_subject,
        };

        subject_and_grades.push(subject_with_grade);
    }

    Ok(subject_and_grades)
}

// format a floating value for display
pub fn format_float(value: f64) -> String {
    format!("{:.2}", value)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
