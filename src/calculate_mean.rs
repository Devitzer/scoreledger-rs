// this function calculates the overall average on your report card
// it uses an array of subjects and then determines your grade with a weighted mean formula
// for the second argument, the grade's position in the array should correspond to the subject's position in that array

use super::subject::SubjectWithGrade;

pub fn calculate_report_mean(subjects_with_grades: Vec<SubjectWithGrade>) -> f32 {
    let mut total_grade_value: f32 = 0.0;
    let mut total_credits: f32 = 0.0;

    // corresponds to a weighted mean, total_grade_value sets up the top, total_credits sets up the bottom
    for subject in subjects_with_grades {
        total_grade_value += subject.grade * subject.subject.value;
        total_credits += subject.subject.value;
    }

    // return the mean of all subjects
    total_grade_value / total_credits
}
