use dialoguer::{Input, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::{process::exit};

use super::errors::ScoreledgerSubjectError;

use super::saving::{save_subject};

#[derive(Serialize, Deserialize, Clone)]
pub struct Subject {
    pub name: String,
    pub value: f32,
}

// A subject select menu which inputs a list of subjects and returns the subject the user selected or nothing if the subject doesn't exist for whatever reason
pub fn prompt_select_subject(subjects: &Vec<Subject>) -> Option<Subject> {
    let mut choices: Vec<String> = vec![];

    for subject in subjects {
        choices.push(subject.name.clone());
    }

    let subject_selection_menu = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a subject to delete")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    let choice = subjects.get(subject_selection_menu);

    choice.cloned()
}

// Prompt users to add a subject
pub fn prompt_subject(save: bool) -> Result<Subject, ScoreledgerSubjectError> {
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
            return Err(ScoreledgerSubjectError::NaNWeight)
        }
    };

    let subject = Subject {
        name: subject_name_input,
        value: subject_weight_float,
    };

    if save {
        // save logic, the error here should be that the subject already exists
        match save_subject(subject.clone()) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }
    };

    Ok(subject)
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
