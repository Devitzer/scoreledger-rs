use dialoguer::{Input, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};

use crate::errors::{ScoreledgerGoalError};
use crate::saving::{save_goal};

#[derive(Serialize, Deserialize, Clone)]
pub struct Goal {
    pub name: String,
    pub threshold: f32,
}

// A goal select menu which inputs a list of goals and returns the goal the user selected or nothing if the goal doesn't exist for whatever reason
pub fn prompt_select_goal(goals: &Vec<Goal>) -> Option<Goal> {
    let mut choices: Vec<String> = vec![];

    for goal in goals {
        choices.push(goal.name.clone());
    }

    let goal_selection_menu = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a goal to delete")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    let choice = goals.get(goal_selection_menu);

    choice.cloned()
}

pub fn prompt_goal(save: bool) -> Result<Goal, ScoreledgerGoalError> {
    let goal_name_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name of goal:".to_string())
        .interact_text()
        .unwrap();

    let goal_threshold_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Goal threshold:".to_string())
        .interact_text()
        .unwrap();

    let goal_threshold_float = match goal_threshold_input.parse::<f32>() {
        Ok(v) => v,
        Err(_) => {
            return Err(ScoreledgerGoalError::NaNThreshold)
        }
    };

    let goal = Goal {
        name: goal_name_input,
        threshold: goal_threshold_float,
    };

    if save {
        // save logic
        save_goal(goal.clone());
    }

    Ok(goal)
}
