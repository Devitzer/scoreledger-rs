use dialoguer::{Input, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};

use super::errors::ScoreledgerSubjectError;

use super::saving::save_subject;

#[derive(Serialize, Deserialize, Clone)]
pub struct Subject {
    pub name: String,
    pub value: f64,
}

// A subject select menu which inputs a list of subjects and returns the subject the user selected
pub fn prompt_select_subject(subjects: &Vec<Subject>) -> Result<Subject, ScoreledgerSubjectError> {
    let mut choices: Vec<String> = vec![];

    for subject in subjects {
        choices.push(subject.name.clone());
    }

    if choices.is_empty() {
        return Err(ScoreledgerSubjectError::NoSubjectExists);
    }

    let subject_selection_menu = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a subject to delete")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    let choice = subjects.get(subject_selection_menu).expect("Unexpected Error: Subject selected in selection menu cannot be found in data (very unexpected, please report)");

    Ok(choice.clone())
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

    let subject_weight_float = match subject_weight_input.parse::<f64>() {
        Ok(v) => v,
        Err(_) => return Err(ScoreledgerSubjectError::NaNWeight),
    };

    let subject = Subject {
        name: subject_name_input,
        value: subject_weight_float,
    };

    if save {
        // save logic, the error here should be that the subject already exists
        match save_subject(subject.clone()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    };

    Ok(subject)
}
