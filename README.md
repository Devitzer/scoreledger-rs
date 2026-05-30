# Scoreledger (EARLY DEVELOPMENT)

Scoreledger is a CLI tool that allows you to create a report card by adding subjects. You can also add goals and view your "report card" to see whether you've met your goals based on your grades. <br>
It's a good tool to have early info on specific details regarding your grades to see if you meet requirements for certain awards, is an example of a use case. You can also keep track of personal goals that you want to exceed.

## Usage

It's simple to get started. Simply install the CLI using this command: `cargo install scoreledger` <br>
Then, you can run the tool by simply typing `scoreledger`, which will bring up a menu with options. We suggest you start by adding your classes.

## Adding classes

To add a class, you select the "Add a subject" option in the select menu, it will prompt you for:
- Name of the class
- The class' weight

Then you can use the enter grades feature to record a grade from the class (use your real life report card)

## Entering grades

To enter grades, you select the "Enter grades" option in the select menu, it will prompt you for all your classes to enter a grade. <br>
Note that your grades do not necessarily have to be a percentage value. The grades will be reported and your "report card" will be ready to view, unless you want to add goals.

## Adding goals

To add a goal, you select the "Set a goal" option in the select menu, it will prompt you for:
- The goal name (a short title)
- The goal threshold (what amount your report card mean must be above for the goal to be met)

## View report card

To view your report card, you select the "View report card" option in the select menu, it shows you:
- Your grade value for each class
- The mean of your report card (using the weights of the class)
- Whether your goals were met

## Delete a goal

To delete a goal, you select the "Delete a goal" option in the select menu. It will prompt you for:
- The goal you want to delete
- A confirmation that you want to delete the goal

Once confirmed, the goal will be removed from the data file.

## Delete a subject

To delete a subject, you select the "Delete a subject" option in the select menu. It will prompt you for:
- The subject you want to delete
- A confirmation that you want to delete the subject

Once confirmed, the subject will be removed from the data file. (**AS OF v0.3.1, GRADES FOR THE SUBJECT ARE NOT DELETED YET, THIS IS PLANNED VERY SOON**)

## Delete all data

To delete all of your data, which includes:
- All subjects
- All grades
- All goals

You must select the "Delete all data" option in the select menu. It will prompt you with a confirmation to ensure whether you want to delete all of your data. Please note that this deletes the save file, and it cannot be recovered once confirmed.

## Exit

To exit, you select the "Exit" option in the select menu, this one is self-explanatory.

# Versioning

Scoreledger uses the semantic versioning standard ([SemVer](https://semver.org/)). In summary, the version number is structured like this: MAJOR.MINOR.PATCH.
- Major updates are updates that introduce breaking changes. With updates like these, you will have information on how to adjust for these breaking changes.
- Minor updates may introduce new features and bug fixes, that are backwards compatible with other versions. In the context of Scoreledger, it would mean a new update doesn't render your data file invalid.
- Patch updates introduce bug fixes primarily. These updates are to fix unintended behaviour.
**Note that Scoreledger is currently in v0, meaning breaking updates can be introduced in minor updates, according to SemVer.**

# Contributing
If you'd like to do anything for Scoreledger, mainly to patch bugs you notice or complete tasks marked TODO in comments or README.md, you can do the following steps:
- Fork the repository and clone it to your machine
- Make your changes
- Make sure they work as expected
- Run cargo clippy and apply changes, then run cargo fmt
- Push to your fork, and then open a pull request

We will review to make sure the steps have been completed as expected and the changes you've applied are as described, along with checking for any malicious code. <br>
Once your pull request is approved, it will be merged to the main branch and applied to the next update.

# TODO

This is a list of general big todo tasks, once these are complete I will consider making the CLI into stable and then introducing new features.

- Make the CLI stable, there is a lot of unhandled edge cases
- Make it a lot more Rust idiomatic, a lot of functions exit whenever they have error, make them return Results instead. Make the errors Enums instead of strings.
- Introduce settings/configuration, specifically one where users can select a unit for their marks, such as %. Since the CLI is designed to be as adaptable as possible, it cannot assume grades are a percentage by default.