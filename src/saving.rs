// handle everything related to saving persistent data, subjects, grades, and goals
// TODO: add more errors for more cases other than "cli doesn't have permission"

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::process::exit;
use std::{fs, path::PathBuf};

use super::errors::{ScoreledgerGoalError, ScoreledgerSubjectError};

use super::goals::Goal;
use super::subject::Subject;
use super::errors;

#[derive(Deserialize, Serialize)]
pub struct Save {
    pub subjects: HashMap<String, Subject>,
    pub goals: HashMap<String, Goal>,
    pub grades: HashMap<String, f32>,
}

fn get_save_dir() -> PathBuf {
    // TODO: put custom error here in the future
    let mut base_dir = config_dir().unwrap();

    base_dir.push("scoreledger_cli");
    fs::create_dir_all(&base_dir).unwrap();
    base_dir
}

pub fn get_data() -> Save {
    let mut save_dir = get_save_dir();
    save_dir.push("data.json");
    let save_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&save_dir)
        .expect("Failed to open data file, the CLI may not have permissions.");

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
            .expect("ERROR: Failed to create ");

        let base_file = "{ \"subjects\": {}, \"goals\": {}, \"grades\": {} }".as_bytes();

        file.write_all(base_file)
            .expect("ERROR: Failed to initialize the data file, the CLI may not have permissions.");
    };

    let save: Save = serde_json::from_reader(save_file).expect("ERROR: Failed to load your saved information. The save file may not be structured as expected.");

    save
}

// this assumes the data exists, because there isn't a situation where you can run this without data existing
// TODO: make this not assume data exists
pub fn write_data(data: Save) {
    let json = serde_json::to_string_pretty(&data).unwrap();
    let mut save_dir = get_save_dir();
    save_dir.push("data.json");
    // debug line
    // println!("Saving to: {:?}", save_dir);

    fs::write(save_dir, json)
        .expect("ERROR: Failed to save data, the CLI may not have necessary permissions.");
}

// this deletes the directory of data pretty much
pub fn delete_all_data() {
    let save_dir = get_save_dir();

    let deletion = match fs::remove_dir_all(save_dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(errors::map_fs_error(e)),
    };

    deletion.unwrap_or_else(|err| {
        // handle that ig
    });

    println!("Your data was deleted successfully.");
}

// deletes a subject from a save by name
// TODO: make it also delete grades related to the subject
pub fn delete_subject(name: String) -> Result<(), ScoreledgerSubjectError> {
    // get existing data
    let mut data = get_data();

    // attempt to find subject
    let subject_search = data.subjects.get(&name);
    // determine if subject was found or not
    if subject_search.is_some() {
        data.subjects.remove(&name);
        write_data(data);
        println!("Successfully deleted the subject \"{}\"", &name);
        Ok(())
    } else {
        Err(ScoreledgerSubjectError::SubjectDoesntExist(name))
    }
}

// deletes a goal from a save by name
pub fn delete_goal(name: String) -> Result<(), ScoreledgerGoalError> {
    // get existing data
    let mut data = get_data();

    // attempt to find goal
    let goal_search = data.goals.get(&name);
    // determine if goal was found or not
    if goal_search.is_some() {
        data.goals.remove(&name);
        write_data(data);
        println!("Successfully deleted the goal \"{}\"", &name);
        Ok(())
    } else {
        Err(ScoreledgerGoalError::GoalDoesntExist(name))
    }
}

pub fn save_subject(subject: Subject) -> Result<(), ScoreledgerSubjectError> {
    // get existing data
    let mut data = get_data();

    // add new data to it (ensure it doesn't already exist)
    if data.subjects.contains_key(&subject.name) {
        return Err(ScoreledgerSubjectError::SubjectAlreadyExists)
    }

    data.subjects.insert(subject.name.clone(), subject);

    // save and finish
    write_data(data);
    Ok(())
}

pub fn save_grades(grades: HashMap<String, f32>) {
    // get existing data
    let mut data = get_data();

    // overwrite data
    data.grades = grades;

    // save and finish
    write_data(data);
}

pub fn save_goal(goal: Goal) -> Result<(), ScoreledgerGoalError> {
    // get existing data
    let mut data = get_data();

    // add new data to it (ensure it doesn't already exist)
    if data.goals.contains_key(&goal.name) {
        return Err(ScoreledgerGoalError::GoalAlreadyExists)
    }

    data.goals.insert(goal.name.clone(), goal);

    // save and finish
    write_data(data);
    Ok(())
}
