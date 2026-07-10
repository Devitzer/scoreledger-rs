use serde::Deserialize;

use crate::errors::ScoreledgerSubjectError;

use super::subject::Subject;

#[derive(Deserialize)]
pub struct SubjectList {
    subjects: Vec<Subject>
}

// get a subject list from json
pub fn get_subject_list(json: String) -> Result<SubjectList, ScoreledgerSubjectError> {
    return serde_json::from_str(json.as_str()).map_err(|_| ScoreledgerSubjectError::InvalidSubjectList);
}

// overwrites the existing subjects in your save with whatever subjects are in the subject list
// this function modifies the save
pub fn rewrite_subjects_with_list(list: SubjectList) {

}