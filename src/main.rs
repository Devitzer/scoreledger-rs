use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::exit;

use scoreledger::{calculate_mean, goals, saving, subject};

fn main() {
    let selections = &[
        "Add a subject",
        "Enter grades",
        "Set a goal",
        "View report card",
        "Delete data",
        "Exit",
    ];

    let selection_menu = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Scoreledger - Select an option")
        .items(&selections[..])
        .default(0)
        .interact()
        .unwrap();

    let choice = selections[selection_menu];

    println!("You selected: {}", choice);

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

        let grades = subject::prompt_grades(subjects);
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
        todo!();
    } else if choice == "View report card" {
        let data = saving::get_data();

        // check each subject that exists and see if a grade exists for it, returns an error saying the subject that is missing a grade
        subject::verify_grades(&data).unwrap_or_else(|msg| {
            println!("{}", msg);
            exit(1);
        });

        let subjects_with_grades = subject::subjects_with_grades(data);

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

        // TODO: determine if goals are met
    } else if choice == "Exit" {
        println!("Exiting...");
        exit(0);
    } else if choice == "Delete data" {
        // confirm user wants to delete data
        let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you really want to continue?")
            .default(true)
            .interact()
            .unwrap();
        // delete stuff
        if confirmation {
            saving::delete_all_data();
        } else {
            println!("Your data has not been deleted.");
        };
    } else if choice == "DEBUG" {
        println!("nothing to see");
    }
}
