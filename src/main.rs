use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::exit;

use scoreledger::{calculate_mean, goals, grades, saving, subject};

fn main() {
    let selections = &[
        "Add a subject",
        "Enter grades",
        "Set a goal",
        "View report card",
        "Delete a subject",
        "Delete a goal",
        "Delete all data",
        "Exit",
    ];

    let selection_menu = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Scoreledger - Select an option")
        .items(&selections[..])
        .default(0)
        .interact()
        .unwrap();

    let choice = selections[selection_menu];

    // determine command to be done
    if choice == "Add a subject" {
        let subject = subject::prompt_subject(true);

        println!(
            "Your new subject \"{}\" was added successfully.",
            &subject.name
        );
    } else if choice == "Enter grades" {
        // get data
        let data = saving::get_data();
        let subjects: Vec<subject::Subject> = data.subjects.into_values().collect();

        let grades = grades::prompt_grades(subjects);
        if grades.is_empty() {
            println!(
                "ERROR: There are no subjects that you can enter grades for! Add a new subject!"
            );
            exit(1);
        }
        // save grades
        saving::save_grades(grades);
        println!("Grades entered successfully! Your report card is ready to view.");
    } else if choice == "Set a goal" {
        let goal = goals::prompt_goal(true);

        println!("Your new goal \"{}\" was added successfully.", &goal.name);
    } else if choice == "Delete a goal" {
        let data = saving::get_data();
        let goals: Vec<goals::Goal> = data.goals.into_values().collect();

        let goal_selection = goals::prompt_select_goal(&goals).expect("ERROR: The goal you selected no longer exists. This is an unexpected error and may mean your data is partially corrupted.");
        let prompt = format!(
            "Are you sure you want to delete the goal \"{}\"?",
            &goal_selection.name
        );

        let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(true)
            .interact()
            .unwrap();

        if confirmation {
            saving::delete_goal(goal_selection.name);
        } else {
            println!("Your data has not been deleted.");
        }
    } else if choice == "View report card" {
        let data = saving::get_data();

        // check each subject that exists and see if a grade exists for it, returns an error saying the subject that is missing a grade
        grades::verify_grades(&data).unwrap_or_else(|msg| {
            println!("{}", msg);
            exit(1);
        });

        let subjects_with_grades = grades::subjects_with_grades(&data);

        // go through each subject and grade and display it to the user
        for subject_and_grade in &subjects_with_grades {
            println!(
                "{}: {:.2}",
                subject_and_grade.subject.name, subject_and_grade.grade
            );
        }

        // display mean of report card to user
        let mean = calculate_mean::calculate_report_mean(subjects_with_grades);
        println!("Report Card Mean: {:.2}", mean);

        // determine if goals are met
        let goal_vec: Vec<goals::Goal> = data.goals.into_values().collect();
        for goal in goal_vec {
            if mean >= goal.threshold {
                println!(
                    "Goal \"{}\" ({:.2} Threshold): ✓",
                    &goal.name, &goal.threshold
                );
            } else {
                println!(
                    "Goal \"{}\" ({:.2} Threshold): ✗",
                    &goal.name, &goal.threshold
                );
            }
        }
    } else if choice == "Exit" {
        println!("Exiting...");
        exit(0);
    } else if choice == "Delete all data" {
        // confirm user wants to delete data
        let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "Do you really want to continue? This will delete all subjects, grades, and goals.",
            )
            .default(true)
            .interact()
            .unwrap();
        // delete stuff
        if confirmation {
            saving::delete_all_data();
        } else {
            println!("Your data has not been deleted.");
        };
    } else if choice == "Delete a subject" {
        let data = saving::get_data();
        let subjects: Vec<subject::Subject> = data.subjects.into_values().collect();

        let subject_selection = subject::prompt_select_subject(&subjects).expect("ERROR: The subject you selected no longer exists. This is an unexpected error and may mean your data is partially corrupted.");
        let prompt = format!(
            "Are you sure you want to delete the subject \"{}\"?",
            &subject_selection.name
        );

        let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(true)
            .interact()
            .unwrap();

        if confirmation {
            saving::delete_subject(subject_selection.name);
        } else {
            println!("Your data has not been deleted.");
        }
    } else if choice == "DEBUG" {
        println!("nothing to see");
    }
}
