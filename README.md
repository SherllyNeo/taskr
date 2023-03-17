A simple rust app for tasks.

USAGE

taskr --add "Go shopping" "2021-06-11"
Added task!

taskr --add "Go gym" "2021-03-11"
Added task!

taskr --done "Go shopping"
Deleted task!

taskr --list

This will lists tasks with the most recent at the bottom of the output


taskr --upcoming
❗3
📅 5


the ! is for tasks due today or tomorrow
the candendar is for tasks due within 7 days

taskr --clear
Deleted all tasks!

taskr --import_ics file_name

This will read an ics file and import the task SUMMARY and just the date from the datetime into the task manager
