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
- Whether your goals were met (**TO BE ADDED**)

## Exit

To exit, you select the "Exit" option in the select menu, this one is self-explanatory.

## Delete a goal

**TO BE ADDED**

## Delete a subject

**TO BE ADDED**

# TODO

This is a list of general big todo tasks, once these are complete I will consider making the CLI into stable and then introducing new features.

- Make the CLI stable, there is a lot of unhandled edge cases
- Make it more Rust idiomatic, a lot of functions exit whenever they have error, make them return Results instead.
- Implement goals