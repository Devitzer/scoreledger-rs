use std::io;

pub enum ScoreledgerFileError {
    FileNotFound,
    FilePermissionDenied,
    FileDirectoryNotEmpty,
    FileAlreadyExists, // the error here and the ones above are simply wrappers of io::ErrorKind errors.
    FileUnknownError,
    FailedToParseSave, // usually because the save file is structured incorrectly
}

pub enum ScoreledgerSubjectError {
    SubjectDoesntExist(String), // when someone tries to do something with a subject that doesn't exist, the name of the subject attempted to find
    SubjectAlreadyExists, // the name of the subject
    NaNWeight, // the weight of a subject person entered wasnt a number
}

pub enum ScoreledgerGoalError {
    GoalDoesntExist(String), // when someone tries to do something with a goal that doesn't exist, returns name of goal attempted to find
    GoalAlreadyExists, // usually when someone tries to add a goal that already exists, returns the goal to show
    NaNThreshold, // the threshold person entered wasnt a number
}

pub enum ScoreledgerGradeError {
    NaNGrade, // the grade person entered wasnt a number
    GradeMissing(String), // grade missing, returns the name of the subject missing a grade
    NoSubjectsForGrades, // when you want to enter grades but no subjects exist to enter grades for
}

// TODO: map io errors to scoreledger errors
pub fn map_fs_error(e: io::Error) -> ScoreledgerFileError {
    match e.kind() {
        io::ErrorKind::NotFound => {
            ScoreledgerFileError::FileNotFound
        }
        io::ErrorKind::PermissionDenied => {
            ScoreledgerFileError::FilePermissionDenied
        }
        io::ErrorKind::DirectoryNotEmpty => {
            ScoreledgerFileError::FileDirectoryNotEmpty
        }
        io::ErrorKind::AlreadyExists => {
            ScoreledgerFileError::FileAlreadyExists
        }
        _ => {
            ScoreledgerFileError::FileUnknownError
        }
    }
}

// returns default error messages for each ScoreledgerFileError
pub fn default_fs_error(err: ScoreledgerFileError) -> String {
    match err {
        ScoreledgerFileError::FileNotFound => "".to_string(),
        ScoreledgerFileError::FilePermissionDenied => "".to_string(),
        ScoreledgerFileError::FileDirectoryNotEmpty => "".to_string(),
        ScoreledgerFileError::FileAlreadyExists => "".to_string(),
        ScoreledgerFileError::FileUnknownError => "".to_string(),
        ScoreledgerFileError::FailedToParseSave => "".to_string()
    }
}