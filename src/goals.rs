use std::process::exit;

use serde::{Serialize, Deserialize};
use dialoguer::{Input, theme::ColorfulTheme};

use crate::saving::save_goal;

#[derive(Serialize, Deserialize, Clone)]
pub struct Goal {
    pub name: String,
    pub threshold: f32
}

pub fn prompt_goal(save: bool) -> Goal {
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
                println!("ERROR: The threshold that you input must be a number, decimals allowed.");
                exit(1);
            }
        };

    let goal = Goal {
        name: goal_name_input,
        threshold: goal_threshold_float
    };

    if save {
        // save logic
        save_goal(goal.clone());
    }

    goal
}