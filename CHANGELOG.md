# v0.4.0 (05/31/2026)

The backend of Scoreledger has been partially rebuilt.

## Added
- New error messages, they're easier to read and more precise.

## Changed
- The backend of Scoreledger has been partially rebuilt. This is to ensure that functions related to working with subjects, goals, grades, saves, etc are dedicated to only that, leaving everything with logging to be handled by the CLI.
- Made the code more idiomatic. Less process::exit functions, use Result instead.
- Logic related to handling grades moved to it's own file (it was formerly in the subjects file.)

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