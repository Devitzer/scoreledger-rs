use std::io;

#[derive(Debug)]
pub enum ScoreledgerFileError {
    FileNotFound,
    FilePermissionDenied,
    FileDirectoryNotEmpty,
    FileAlreadyExists, // the error here and the ones above are simply wrappers of io::ErrorKind errors.
    FileUnknownError(io::Error),
    SaveDirectoryNotFound,
    FailedToParseSave, // usually because the save file is structured incorrectly
}

#[derive(Debug)]
pub enum ScoreledgerSubjectError {
    SubjectDoesntExist(String), // when someone tries to do something with a subject that doesn't exist, the name of the subject attempted to find
    SubjectAlreadyExists,       // the name of the subject
    NoSubjectExists, // typically when you are trying to make a select menu of subjects but none exist
    NaNWeight,       // the weight of a subject person entered wasnt a number
    InvalidSubjectList, // when you try to import a subject list from a json file and it does not follow the correct structure
}

#[derive(Debug)]
pub enum ScoreledgerGoalError {
    GoalDoesntExist(String), // when someone tries to do something with a goal that doesn't exist, returns name of goal attempted to find
    GoalAlreadyExists, // usually when someone tries to add a goal that already exists, returns the goal to show
    NoGoalExists, // typically when you are trying to make a select menu of goals but none exist
    NaNThreshold, // the threshold person entered wasnt a number
}

#[derive(Debug)]
pub enum ScoreledgerGradeError {
    NaNGrade,             // the grade person entered wasnt a number
    GradeMissing(String), // grade missing, returns the name of the subject missing a grade
    NoSubjectsForGrades,  // when you want to enter grades but no subjects exist to enter grades for
}

pub fn map_fs_error(e: io::Error) -> ScoreledgerFileError {
    match e.kind() {
        io::ErrorKind::NotFound => ScoreledgerFileError::FileNotFound,
        io::ErrorKind::PermissionDenied => ScoreledgerFileError::FilePermissionDenied,
        io::ErrorKind::DirectoryNotEmpty => ScoreledgerFileError::FileDirectoryNotEmpty,
        io::ErrorKind::AlreadyExists => ScoreledgerFileError::FileAlreadyExists,
        _ => ScoreledgerFileError::FileUnknownError(e),
    }
}

// returns default error messages for each ScoreledgerFileError
pub fn default_fs_error(err: ScoreledgerFileError) -> String {
    match err {
        ScoreledgerFileError::FileNotFound => "ERROR: Files/directories related to the save were not found. They may have been deleted.".to_string(),
        ScoreledgerFileError::FilePermissionDenied => "ERROR: Scoreledger doesn't have the necessary permission to access your save.".to_string(),
        ScoreledgerFileError::FileDirectoryNotEmpty => "ERROR: Cannot delete save directory, because it's not empty. (UNEXPECTED ERROR, PLEASE REPORT)".to_string(),
        ScoreledgerFileError::FileAlreadyExists => "ERROR: Attempted to recreate something that already exists. (UNEXPECTED ERROR, PLEASE REPORT)".to_string(),
        ScoreledgerFileError::FileUnknownError(e) => format!("ERROR: Unknown file-related error | {}", e),
        ScoreledgerFileError::FailedToParseSave => "ERROR: Failed to parse save. The save file is likely partially corrrupted.".to_string(),
        ScoreledgerFileError::SaveDirectoryNotFound => "ERROR: Failed to retrieve save directory.".to_string()
    }
}

// default error messages worded to consider the most common situations they would appear
pub fn default_subject_error(err: ScoreledgerSubjectError) -> String {
    match err {
        ScoreledgerSubjectError::SubjectDoesntExist(name) => {
            format!("ERROR: The subject \"{}\" does not exist!", name)
        }
        ScoreledgerSubjectError::SubjectAlreadyExists => {
            "ERROR: The subject you are trying to add already exists!".to_string()
        }
        ScoreledgerSubjectError::NaNWeight => {
            "ERROR: The weight you entered for the subject is not a number!".to_string()
        }
        ScoreledgerSubjectError::NoSubjectExists => {
            "ERROR: You cannot select a subject to delete because you haven't made any subjects!".to_string()
        }
        ScoreledgerSubjectError::InvalidSubjectList => {
            "ERROR: The subject list you provided does not follow the correct structure.".to_string()
        }
    }
}

pub fn default_goal_error(err: ScoreledgerGoalError) -> String {
    match err {
        ScoreledgerGoalError::GoalDoesntExist(name) => {
            format!("ERROR: The goal \"{}\" does not exist!", name)
        }
        ScoreledgerGoalError::GoalAlreadyExists => {
            "ERROR: The goal you are trying to add already exists!".to_string()
        }
        ScoreledgerGoalError::NaNThreshold => {
            "ERROR: The threshold you entered for the goal is not a number!".to_string()
        }
        ScoreledgerGoalError::NoGoalExists => {
            "ERROR: You cannot select a goal to delete because you haven't made any goals!"
                .to_string()
        }
    }
}

pub fn default_grade_error(err: ScoreledgerGradeError) -> String {
    match err {
        ScoreledgerGradeError::GradeMissing(name) => format!("ERROR: The subject \"{}\" is missing a grade! Please run \"Enter grades\" before viewing report card.", name),
        ScoreledgerGradeError::NoSubjectsForGrades => "ERROR: There is no grades to enter because there are no subjects! Please add a subject before trying to enter grades.".to_string(),
        ScoreledgerGradeError::NaNGrade => "ERROR: The grade you entered was not a number!".to_string()
    }
}

pub fn is_url(input: &str) -> bool {
    input.starts_with("http://") || input.starts_with("https://")
}