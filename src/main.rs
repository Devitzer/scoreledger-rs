use std::process::exit;
use dialoguer::{Select, theme::ColorfulTheme};

use scoreledger::{calculate_mean, goals, saving, subject};

fn main() {
    let selections = &[
        "Add a subject",
        "Enter grades",
        "Set a goal",
        "Delete a goal",
        "View report card",
        "DEBUG",
        "Exit"
    ];

    let selection_menu = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Scoreledger - Select an option")
        .items(&selections[..])
        .default(6)
        .interact()
        .unwrap();

    let choice = selections[selection_menu];

    println!("You selected: {}", choice);

    // determine command to be done
    if choice == "Add a subject" {
        let subject = subject::prompt_subject(true);

        println!("Your new subject \"{}\" was added successfully.", &subject.name);
    } else if choice == "Enter grades" {
        // get data
        let data = saving::get_data();
        let subjects: Vec<subject::Subject> = data.subjects.into_values().collect();

        let grades = subject::prompt_grades(subjects);
        // save grades
        saving::save_grades(grades);
        println!("Grades entered successfully! Your report card is ready to view.");
    } else if choice == "Set a goal" {
        let goal = goals::prompt_goal(true);

        println!("Your new goal \"{}\" was added successfully.", &goal.name);
    } else if choice == "Delete a goal" {
        todo!();
    } else if choice == "View report card" {
        let data = saving::get_data();
        let subjects_with_grades = subject::subjects_with_grades(data);

        // go through each subject and grade and display it to the user
        for subject_and_grade in &subjects_with_grades {
            println!("{}: {}", subject_and_grade.subject.name, subject_and_grade.grade);
        }

        // display mean of report card to user
        let mean = calculate_mean::calculate_report_mean(subjects_with_grades);
        println!("Report Card Mean: {}", mean);
        
        // TODO: determine if goals are met
        
    } else if choice == "Exit" {
        println!("Exiting...");
        exit(0);
    } else if choice == "DEBUG" {
        println!("nothing to see");
    }
}
