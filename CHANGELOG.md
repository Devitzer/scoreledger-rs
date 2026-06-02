# v0.5.0 (06/02/2026)

Completed a bunch of small TODO tasks.

## Added
- There is a new Settings menu, where you can configure certain options. There is currently one, where you can change the unit that appears after the score number.
- Added new "Settings" section to Save struct, which is a HashMap where the key is the setting and the value is the setting's value. (to be completed)
- Decimal scores will no longer show trailing zeros.
- Completed TODO tasks labelled in source code (most were already complete but not removed)
- New ScoreledgerFileError "SaveDirectoryNotFound", it is returned when the save directory can't be retrieved.

## Bug Fixes
- Failing to retrieve save directory now returns a more readable error.

# v0.4.0 (05/31/2026)

The backend of Scoreledger has been partially rebuilt. This is one major step to reaching stability.

## Added
- New error messages, they're easier to read and more precise.
- Removing a subject also removes the grade related to it.

## Changed
- The backend of Scoreledger has been partially rebuilt. This is to ensure that functions related to working with subjects, goals, grades, saves, etc are dedicated to only that, leaving everything with logging to be handled by the CLI.
- Made the code more idiomatic. Less process::exit functions, use Result instead.
- Logic related to handling grades moved to it's own file (it was formerly in the subjects file.)

## Bug Fixes
- "Delete a subject" option no longer panics when there are no subjects to delete.
- "Delete a goal" option no longer panics when there are no goals to delete.

# v0.3.1 (05/30/2026)

## Fixes
- Adjust README.md to reflect new menu options, and change the order slightly.

# v0.3.0 (05/30/2026)

Goals are now fully implemented. The next goal is to make the code more Rust idiomatic and make Scoreledger's function more library friendly (even though they are not really intended to be used).

## Added
- Goals are now checked in the "View report card" option.
- You can now remove individual subjects.
- You can now remove individual goals.

## Changed
- The function that maps subjects to a grade now uses a reference to the Save instead of taking ownership. (Adjustment for goal implementation)
- "Enter grades" now adds any existing grades as a default number you can enter for a subject. Whenever you enter grades, you no longer have to re-enter for certain classes, you now only have to confirm.
- Clarify that the "Delete data" option deletes all data by renaming it "Delete all data".

## Bug Fixes
- Removed an unintended debug line "You selected: (option)".

# v0.2.0 (05/30/2026)

## Added

- "Delete data" option where you can delete all saved data related to the CLI.
- TODO section in README.md.
- New errors related to file tasks which will more precisely tell a user what went wrong. **Not implemented to all function as of yet.**

## Changed
- "View report card" option now checks whether grades exist for all subjects before showing.
- "View report card" handles the situation where no data exists more clearly.
- Grade values on your report card now only show two decimal places to avoid large numbers being displayed.

## Bug Fixes

- Fixed a bug where data was being deleted everytime you try to add data.

# v0.1.0 (05/29/2026)

Initial release.