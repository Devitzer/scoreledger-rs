// handle everything related to saving persistent data, subjects, grades, and goals

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::{fs, path::PathBuf};

use super::errors::{ScoreledgerFileError, ScoreledgerGoalError, ScoreledgerSubjectError};

use super::errors;
use super::goals::Goal;
use super::subject::Subject;

#[derive(Deserialize, Serialize)]
pub struct Save {
    pub subjects: HashMap<String, Subject>,
    pub goals: HashMap<String, Goal>,
    pub grades: HashMap<String, f32>,
    pub settings: HashMap<String, String>,
}

fn get_save_dir() -> Result<PathBuf, ScoreledgerFileError> {
    let mut base_dir = config_dir().ok_or(ScoreledgerFileError::SaveDirectoryNotFound)?;

    base_dir.push("scoreledger_cli");
    fs::create_dir_all(&base_dir).unwrap();
    Ok(base_dir)
}

pub fn get_data() -> Result<Save, ScoreledgerFileError> {
    let mut save_dir = get_save_dir().expect("Unexpected Error: Failed to retrieve save directory");
    save_dir.push("data.json");
    let save_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&save_dir)
        .map_err(errors::map_fs_error)?;

    let is_empty = match fs::metadata(&save_dir) {
        Ok(meta) => meta.len() == 0,
        Err(_) => true, // file doesn't exist
    };

    if is_empty {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&save_dir)
            .map_err(errors::map_fs_error)?;

        let base_file = "{ \"subjects\": {}, \"goals\": {}, \"grades\": {}, \"settings\": {} }".as_bytes();

        file.write_all(base_file).map_err(errors::map_fs_error)?;
    };

    let save: Save =
        serde_json::from_reader(save_file).map_err(|_| ScoreledgerFileError::FailedToParseSave)?;

    Ok(save)
}

// this assumes the data exists, because there isn't a situation where you can run this without data existing
pub fn write_data(data: Save) -> Result<(), ScoreledgerFileError> {
    let json = serde_json::to_string_pretty(&data).unwrap();
    let mut save_dir = get_save_dir().expect("Unexpected Error: Failed to retrieve save directory");
    save_dir.push("data.json");
    // debug line
    // println!("Saving to: {:?}", save_dir);

    fs::write(save_dir, json).map_err(errors::map_fs_error)
}

// this deletes the directory of data pretty much
pub fn delete_all_data() -> Result<(), ScoreledgerFileError> {
    let save_dir = get_save_dir().expect("Unexpected Error: Failed to retrieve save directory");

    match fs::remove_dir_all(save_dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(errors::map_fs_error(e)),
    }
}

// deletes a subject from a save by name
pub fn delete_subject(name: &String) -> Result<(), ScoreledgerSubjectError> {
    // get existing data
    let mut data = get_data().expect("Unexpected error: Failed to load save for subject deletion");

    // attempt to find subject
    let subject_search = data.subjects.get(name);
    // determine if subject was found or not
    if subject_search.is_some() {
        data.subjects.remove(name);
        // get grade for subject, ignore removing it if you cant find (since its already gone)
        data.grades.remove(name);
        write_data(data).expect("Unexpected Error: Failed to write data to save for removing the subject. The subject was not removed.");
        Ok(())
    } else {
        Err(ScoreledgerSubjectError::SubjectDoesntExist(name.clone()))
    }
}

// deletes a goal from a save by name
pub fn delete_goal(name: &String) -> Result<(), ScoreledgerGoalError> {
    // get existing data
    let mut data = get_data().expect("Unexpected error: Failed to load save for goal deletion");

    // attempt to find goal
    let goal_search = data.goals.get(name);
    // determine if goal was found or not
    if goal_search.is_some() {
        data.goals.remove(name);
        write_data(data).expect("Unexpected Error: Failed to write data to save for removing the goal. The goal was not removed.");
        Ok(())
    } else {
        Err(ScoreledgerGoalError::GoalDoesntExist(name.clone()))
    }
}

pub fn save_subject(subject: Subject) -> Result<(), ScoreledgerSubjectError> {
    // get existing data
    let mut data = get_data().expect("Unexpected Error: Failed to load save to add new subject");

    // add new data to it (ensure it doesn't already exist)
    if data.subjects.contains_key(&subject.name) {
        return Err(ScoreledgerSubjectError::SubjectAlreadyExists);
    }

    data.subjects.insert(subject.name.clone(), subject);

    // save and finish
    write_data(data).expect("Unexpected Error: Failed to write data to save new subject");
    Ok(())
}

// TODO: enforce a more specific setting structure in the future (other than mapping string keys to string values)
pub fn save_setting(settings: HashMap<String, String>) {
    // get existing data
    let mut data = get_data().expect("Unexpected Error: Failed to load save to add new setting");

    // overwrite data
    data.settings = settings;

    // save and finish
    write_data(data).expect("Unexpected Error: Failed to write data to save new setting");
}

pub fn save_grades(grades: HashMap<String, f32>) {
    // get existing data
    let mut data = get_data().expect("Unexpected Error: Failed to load save to enter grades");

    // overwrite data
    data.grades = grades;

    // save and finish
    write_data(data).expect("Unexpected Error: Failed to write data to save entered grades");
}

pub fn save_goal(goal: Goal) -> Result<(), ScoreledgerGoalError> {
    // get existing data
    let mut data = get_data().expect("Unexpected Error: Failed to load save to add a goal");

    // add new data to it (ensure it doesn't already exist)
    if data.goals.contains_key(&goal.name) {
        return Err(ScoreledgerGoalError::GoalAlreadyExists);
    }

    data.goals.insert(goal.name.clone(), goal);

    // save and finish
    write_data(data).expect("Unexpected Error: Failed to write data to save goal");
    Ok(())
}
